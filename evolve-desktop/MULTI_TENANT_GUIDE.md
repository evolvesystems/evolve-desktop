# Multi-Tenant Database Architecture

## Overview

For **SaaS deployments**, you can have multiple companies/users in ONE database with proper isolation.

## Database Design

### Add Tenant/Organization to All Tables

```sql
-- Organizations table
CREATE TABLE organizations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    subdomain VARCHAR(100) UNIQUE,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Users belong to organizations
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    organization_id INTEGER NOT NULL REFERENCES organizations(id),
    ...
);

-- Emails are scoped to organizations
CREATE TABLE emails (
    id SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id),
    user_id INTEGER REFERENCES users(id),
    subject VARCHAR(500),
    body TEXT,
    ...
);

-- Index for fast tenant queries
CREATE INDEX idx_emails_org ON emails(organization_id);
CREATE INDEX idx_users_org ON users(organization_id);
```

## EIQ Manager Multi-Tenant Support

### 1. Tenant Context Service

```php
// src/Service/TenantContext.php
class TenantContext
{
    private ?Organization $currentOrganization = null;

    public function setOrganization(Organization $org): void
    {
        $this->currentOrganization = $org;
    }

    public function getOrganization(): Organization
    {
        if (!$this->currentOrganization) {
            throw new \RuntimeException('No organization context set');
        }
        return $this->currentOrganization;
    }

    public function getOrganizationId(): int
    {
        return $this->getOrganization()->getId();
    }
}
```

### 2. Request Listener (Auto-set tenant from JWT)

```php
// src/EventListener/TenantListener.php
class TenantListener
{
    public function __construct(
        private TenantContext $tenantContext,
        private EntityManagerInterface $em
    ) {}

    public function onKernelRequest(RequestEvent $event): void
    {
        // Get user from security context
        $user = $this->security->getUser();

        if ($user) {
            // Load organization from user
            $organization = $user->getOrganization();
            $this->tenantContext->setOrganization($organization);
        }
    }
}
```

### 3. Repository with Automatic Filtering

```php
// src/Repository/EmailRepository.php
class EmailRepository extends ServiceEntityRepository
{
    public function __construct(
        ManagerRegistry $registry,
        private TenantContext $tenantContext
    ) {
        parent::__construct($registry, Email::class);
    }

    public function findAllForCurrentTenant(): array
    {
        return $this->createQueryBuilder('e')
            ->where('e.organization = :org')
            ->setParameter('org', $this->tenantContext->getOrganization())
            ->getQuery()
            ->getResult();
    }

    public function findOneByIdForCurrentTenant(int $id): ?Email
    {
        return $this->createQueryBuilder('e')
            ->where('e.id = :id')
            ->andWhere('e.organization = :org')
            ->setParameter('id', $id)
            ->setParameter('org', $this->tenantContext->getOrganization())
            ->getQuery()
            ->getOneOrNullResult();
    }
}
```

### 4. Controller Updates

```php
// src/Controller/Api/EmailApiController.php
#[Route('/api/v1/emails', name: 'api_email_')]
#[IsGranted('ROLE_USER')]
class EmailApiController extends AbstractController
{
    public function __construct(
        private EmailRepository $emailRepo,
        private TenantContext $tenantContext
    ) {}

    #[Route('', name: 'list', methods: ['GET'])]
    public function list(): JsonResponse
    {
        // Automatically filtered by tenant
        $emails = $this->emailRepo->findAllForCurrentTenant();

        return $this->json([
            'emails' => $emails,
            'organization' => [
                'id' => $this->tenantContext->getOrganizationId(),
                'name' => $this->tenantContext->getOrganization()->getName()
            ]
        ]);
    }
}
```

## Authentication with Organizations

### Register Endpoint (Create Organization)

```php
#[Route('/api/v1/auth/register', methods: ['POST'])]
public function register(Request $request): JsonResponse
{
    $data = json_decode($request->getContent(), true);

    // Create organization
    $organization = new Organization();
    $organization->setName($data['company_name']);
    $organization->setSubdomain($this->generateSubdomain($data['company_name']));
    $this->em->persist($organization);

    // Create user in that organization
    $user = new User();
    $user->setEmail($data['email']);
    $user->setOrganization($organization);
    $user->setPassword($this->passwordHasher->hash($data['password']));
    $this->em->persist($user);

    $this->em->flush();

    return $this->json([
        'user' => [...],
        'organization' => [
            'id' => $organization->getId(),
            'name' => $organization->getName(),
            'subdomain' => $organization->getSubdomain()
        ]
    ]);
}
```

### JWT Token Includes Organization

```php
// Token payload
{
    "user_id": 123,
    "organization_id": 5,  // ← Important!
    "email": "user@company.com",
    "exp": 1234567890
}
```

## Desktop App Changes (Minimal)

The desktop app **doesn't need to know** about organizations!

It just:
1. Logs in with email/password
2. Gets JWT token (which includes organization_id)
3. All API calls automatically scoped to that organization

```typescript
// Desktop app just calls:
await axios.get('/api/v1/emails')

// Backend automatically filters by organization from JWT
```

## Organization Switching (Optional)

For users in multiple organizations:

```typescript
// Desktop app
async function switchOrganization(orgId: number) {
  const response = await axios.post('/api/v1/auth/switch-organization', {
    organization_id: orgId
  })

  // Get new token for that organization
  authStore.setTokens(response.data.access_token)
}
```

## Data Isolation Benefits

### Security
✅ Users can only see their organization's data
✅ SQL injection won't leak other orgs' data
✅ Each org is logically separated

### Performance
✅ Indexes on organization_id
✅ Fast queries with proper filtering
✅ Can partition tables by org if needed

### Compliance
✅ GDPR: Each org's data clearly separated
✅ Data export: Easy to extract one org
✅ Data deletion: Delete by organization_id

## Deployment Models Comparison

| Model | Desktop App Connects To | Database | Tenant Isolation |
|-------|------------------------|----------|------------------|
| **SaaS (Multi-Tenant)** | api.evolveapp.com | Your 1 DB | Via organization_id |
| **Self-Hosted** | customer.com/api | Customer's DB | Not needed (separate DB) |
| **Hybrid** | User chooses | Multiple DBs | Depends on choice |

## Migration Path

### Current Setup (Single Tenant)
```
User → localhost:8547 → Local PostgreSQL
```

### Add Multi-Tenancy
1. Add `organization_id` to all tables
2. Create Organizations table
3. Add TenantContext service
4. Update all repositories
5. Update JWT to include org

### Result
```
User A → api.evolveapp.com → Shared DB (org_id: 1)
User B → api.evolveapp.com → Shared DB (org_id: 2)
User C → company.com/api → Their own DB
```

## When to Use Which Model?

### Use Multi-Tenant (1 Database) When:
- ✅ You want SaaS like Gmail/Slack
- ✅ You manage all infrastructure
- ✅ Users don't need data sovereignty
- ✅ You want simplified operations

### Use Self-Hosted (Many Databases) When:
- ✅ Enterprise customers want control
- ✅ Compliance requires on-premise
- ✅ Customers pay for licenses
- ✅ You want to offer both SaaS + Enterprise

### Use Hybrid (Best of Both) When:
- ✅ You want maximum flexibility
- ✅ Small customers use your cloud
- ✅ Large customers self-host
- ✅ You have the wizard in place ✅

## Your Current Setup

Right now you have:
- ✅ Wizard that lets users choose server
- ✅ EIQ Manager with PostgreSQL on DigitalOcean
- ✅ Desktop app that connects to any API URL

**Next Steps:**
1. **Keep it simple**: Use current setup for self-hosted or single-tenant
2. **Add multi-tenancy**: If you want SaaS with multiple companies in one DB
3. **Or both**: Offer SaaS (multi-tenant) + Enterprise (self-hosted)

**Recommendation**: Start without multi-tenancy, add it later when you have multiple SaaS customers!
