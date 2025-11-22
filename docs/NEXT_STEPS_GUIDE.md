# Next Steps: Implementing Smart Sync Strategy

This guide provides step-by-step instructions for implementing the smart synchronization strategy between EIQ Manager and the desktop app.

---

## Immediate Actions (This Week)

### 1. Install OpenAPI Documentation Bundle

```bash
cd /Users/jamesnorth/sources/eiq-manager

# Install nelmio/api-doc-bundle
composer require nelmio/api-doc-bundle

# The bundle will auto-configure via Symfony Flex
```

### 2. Configure API Documentation

Create configuration file:

```bash
# Create config file
cat > config/packages/nelmio_api_doc.yaml << 'EOF'
nelmio_api_doc:
    documentation:
        info:
            title: EIQ Manager API
            description: |
                REST API for EvolveApp Desktop and Mobile clients.

                ## Authentication
                All endpoints (except /auth) require Bearer token authentication.
                Include the JWT token in the Authorization header:
                `Authorization: Bearer YOUR_TOKEN`

                ## Rate Limiting
                Currently no rate limiting. May be added in future versions.

                ## Versioning
                API version is specified in the URL path: `/api/v1/`
                Breaking changes will increment the major version: `/api/v2/`
            version: 1.0.0
            contact:
                email: support@evolvepreneuriq.app
        servers:
            - url: http://localhost:8547
              description: Local development server
            - url: https://evolvepreneuriq.app
              description: Production server
        components:
            securitySchemes:
                bearerAuth:
                    type: http
                    scheme: bearer
                    bearerFormat: JWT
                    description: 'JWT token from /api/v1/auth/login endpoint'
        security:
            - bearerAuth: []

    areas:
        path_patterns:
            - ^/api(?!/doc$) # All routes starting with /api except /api/doc

    models:
        names:
            - { alias: Email, type: App\Entity\Email\EmailMessage }
            - { alias: EmailFolder, type: App\Entity\Email\EmailFolder }
            - { alias: User, type: App\Entity\User }
            - { alias: Contact, type: App\Entity\CRMMarketing\Contact }
            - { alias: Book, type: App\Entity\EvolveWriter\Book }
            - { alias: Chapter, type: App\Entity\EvolveWriter\Chapter }
EOF
```

### 3. Add OpenAPI Annotations to Existing Controllers

Update `src/Controller/Api/EmailApiController.php`:

```php
<?php
// Add these imports at the top
use OpenApi\Attributes as OA;

// Add class-level documentation
#[OA\Tag(name: 'Email', description: 'Email management endpoints')]
#[Route('/api/v1/emails', name: 'api_email_')]
#[IsGranted('ROLE_USER')]
class EmailApiController extends AbstractController
{
    // Add to list() method:
    #[OA\Get(
        path: '/api/v1/emails',
        summary: 'List emails with filtering and pagination',
        description: 'Retrieve a paginated list of emails for the authenticated user. Supports various filters.',
        tags: ['Email'],
        parameters: [
            new OA\Parameter(
                name: 'page',
                in: 'query',
                description: 'Page number for pagination',
                required: false,
                schema: new OA\Schema(type: 'integer', default: 1, minimum: 1)
            ),
            new OA\Parameter(
                name: 'limit',
                in: 'query',
                description: 'Number of items per page',
                required: false,
                schema: new OA\Schema(type: 'integer', default: 50, minimum: 1, maximum: 100)
            ),
            new OA\Parameter(
                name: 'folder_id',
                in: 'query',
                description: 'Filter by folder ID',
                required: false,
                schema: new OA\Schema(type: 'integer')
            ),
            new OA\Parameter(
                name: 'is_read',
                in: 'query',
                description: 'Filter by read status',
                required: false,
                schema: new OA\Schema(type: 'boolean')
            ),
            new OA\Parameter(
                name: 'search',
                in: 'query',
                description: 'Search in subject, from, and content',
                required: false,
                schema: new OA\Schema(type: 'string')
            )
        ],
        responses: [
            new OA\Response(
                response: 200,
                description: 'List of emails',
                content: new OA\JsonContent(
                    properties: [
                        new OA\Property(
                            property: 'data',
                            type: 'array',
                            items: new OA\Items(
                                properties: [
                                    new OA\Property(property: 'id', type: 'integer'),
                                    new OA\Property(property: 'subject', type: 'string'),
                                    new OA\Property(property: 'from_address', type: 'string'),
                                    new OA\Property(property: 'to_address', type: 'string'),
                                    new OA\Property(property: 'received_date', type: 'string', format: 'date-time'),
                                    new OA\Property(property: 'is_read', type: 'boolean'),
                                    new OA\Property(property: 'is_flagged', type: 'boolean'),
                                    new OA\Property(property: 'has_attachments', type: 'boolean')
                                ]
                            )
                        ),
                        new OA\Property(
                            property: 'meta',
                            properties: [
                                new OA\Property(property: 'page', type: 'integer'),
                                new OA\Property(property: 'limit', type: 'integer'),
                                new OA\Property(property: 'total', type: 'integer'),
                                new OA\Property(property: 'pages', type: 'integer')
                            ],
                            type: 'object'
                        )
                    ]
                )
            ),
            new OA\Response(
                response: 401,
                description: 'Unauthorized - Invalid or missing token',
                content: new OA\JsonContent(
                    properties: [
                        new OA\Property(property: 'error', type: 'string', example: 'Unauthorized')
                    ]
                )
            )
        ],
        security: [['bearerAuth' => []]]
    )]
    #[Route('', name: 'list', methods: ['GET'])]
    public function list(Request $request): JsonResponse
    {
        // existing code...
    }
}
```

### 4. Generate OpenAPI Spec

```bash
cd /Users/jamesnorth/sources/eiq-manager

# Generate JSON spec
php bin/console nelmio:apidoc:dump --format=json > public/api/openapi.json

# Generate YAML spec
php bin/console nelmio:apidoc:dump --format=yaml > public/api/openapi.yaml

# View in browser (start server if needed)
symfony server:start -d
open http://localhost:8547/api/doc
```

### 5. Set Up Type Generation in Desktop App

```bash
cd /Users/jamesnorth/sources/evolveapp/evolve-desktop

# Install openapi-typescript
npm install --save-dev openapi-typescript

# Add to package.json scripts
npm pkg set scripts.types:generate="openapi-typescript http://localhost:8547/api/openapi.json -o src/types/api-generated.ts"
npm pkg set scripts.types:generate:prod="openapi-typescript https://evolvepreneuriq.app/api/openapi.json -o src/types/api-generated.ts"

# Generate types for the first time
npm run types:generate
```

### 6. Create Type-Safe API Service

Create `src/services/api/typed-client.ts`:

```typescript
import type { paths, components } from '@/types/api-generated'
import axios, { type AxiosInstance } from 'axios'

// Type aliases for convenience
export type EmailMessage = components['schemas']['EmailMessage']
export type EmailFolder = components['schemas']['EmailFolder']
export type User = components['schemas']['User']
export type Contact = components['schemas']['Contact']

// Extract request/response types from paths
type ListEmailsParams = paths['/api/v1/emails']['get']['parameters']['query']
type ListEmailsResponse = paths['/api/v1/emails']['get']['responses']['200']['content']['application/json']

export class TypedApiClient {
  private client: AxiosInstance

  constructor(baseURL: string, token?: string) {
    this.client = axios.create({
      baseURL,
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
        ...(token ? { 'Authorization': `Bearer ${token}` } : {})
      }
    })
  }

  // Email endpoints with full type safety
  async listEmails(params?: ListEmailsParams): Promise<ListEmailsResponse> {
    const { data } = await this.client.get<ListEmailsResponse>('/api/v1/emails', {
      params
    })
    return data
  }

  async getEmail(id: number): Promise<EmailMessage> {
    const { data } = await this.client.get<EmailMessage>(`/api/v1/emails/${id}`)
    return data
  }

  async sendEmail(email: {
    to: string
    subject: string
    body: string
  }): Promise<EmailMessage> {
    const { data } = await this.client.post<EmailMessage>('/api/v1/emails/send', email)
    return data
  }

  // Add more type-safe methods as needed...
}

// Export singleton instance
let apiClient: TypedApiClient | null = null

export function getApiClient(): TypedApiClient {
  if (!apiClient) {
    const baseURL = localStorage.getItem('api_url') || import.meta.env.VITE_API_URL
    const token = localStorage.getItem('access_token')
    apiClient = new TypedApiClient(baseURL, token || undefined)
  }
  return apiClient
}

export function resetApiClient(): void {
  apiClient = null
}
```

---

## Next Week Actions

### 7. Add API Version Headers

Create `src/Controller/Api/BaseApiController.php`:

```php
<?php

namespace App\Controller\Api;

use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\JsonResponse;

abstract class BaseApiController extends AbstractController
{
    protected const API_VERSION = '1.0.0';

    protected function apiResponse(
        mixed $data,
        int $status = 200,
        array $headers = [],
        ?string $deprecationMessage = null,
        ?string $sunsetDate = null
    ): JsonResponse {
        $responseHeaders = array_merge([
            'X-API-Version' => self::API_VERSION,
        ], $headers);

        if ($deprecationMessage) {
            $responseHeaders['X-API-Deprecation-Warning'] = $deprecationMessage;
        }

        if ($sunsetDate) {
            $responseHeaders['X-API-Sunset-Date'] = $sunsetDate;
        }

        return $this->json($data, $status, $responseHeaders);
    }
}
```

Update all API controllers to extend `BaseApiController` and use `apiResponse()`.

### 8. Set Up Automated API Diff Detection

Create `.github/workflows/api-diff-detection.yml`:

```yaml
name: API Diff Detection

on:
  pull_request:
    paths:
      - 'src/Controller/Api/**'
  push:
    branches: [master]
    paths:
      - 'src/Controller/Api/**'

jobs:
  detect-changes:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v3
        with:
          fetch-depth: 0

      - name: Setup PHP
        uses: shivammathur/setup-php@v2
        with:
          php-version: '8.2'
          extensions: mbstring, xml, ctype, iconv, intl

      - name: Install dependencies
        run: composer install --prefer-dist --no-progress

      - name: Generate current OpenAPI spec
        run: |
          php bin/console nelmio:apidoc:dump --format=json > openapi-new.json

      - name: Download previous spec
        run: |
          curl -f https://evolvepreneuriq.app/api/openapi.json > openapi-old.json || echo '{}' > openapi-old.json

      - name: Compare specs
        id: diff
        run: |
          if [ -f openapi-old.json ]; then
            # Use jq to format and compare
            diff <(jq --sort-keys . openapi-old.json) <(jq --sort-keys . openapi-new.json) > api-diff.txt || true

            if [ -s api-diff.txt ]; then
              echo "has_changes=true" >> $GITHUB_OUTPUT
            else
              echo "has_changes=false" >> $GITHUB_OUTPUT
            fi
          fi

      - name: Comment on PR if changes detected
        if: steps.diff.outputs.has_changes == 'true' && github.event_name == 'pull_request'
        uses: actions/github-script@v6
        with:
          script: |
            const fs = require('fs');
            const diff = fs.readFileSync('api-diff.txt', 'utf8');

            github.rest.issues.createComment({
              owner: context.repo.owner,
              repo: context.repo.repo,
              issue_number: context.issue.number,
              body: `## ⚠️ API Changes Detected\n\nThis PR modifies the API. Review carefully and update:\n\n1. API_CHANGELOG.md\n2. Desktop app types\n3. Documentation\n\n<details>\n<summary>View diff</summary>\n\n\`\`\`diff\n${diff.substring(0, 5000)}\n\`\`\`\n</details>`
            });
```

### 9. Create GitHub Action to Notify Desktop Team

Create `.github/workflows/notify-desktop-team.yml`:

```yaml
name: Notify Desktop Team

on:
  push:
    branches: [master]
    paths:
      - 'src/Controller/Api/**'
      - 'docs/API_CHANGELOG.md'

jobs:
  notify:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Create issue in desktop repo
        uses: actions/github-script@v6
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          script: |
            // Get latest commit message
            const commit = context.payload.head_commit;

            // Create issue in desktop app repo
            await github.rest.repos.createDispatchEvent({
              owner: 'evolvesystems',
              repo: 'evolve-desktop',
              event_type: 'api-updated',
              client_payload: {
                commit_sha: commit.id,
                commit_message: commit.message,
                commit_url: commit.url
              }
            });
```

---

## Monthly Maintenance Tasks

### Update Feature Parity Matrix

1. Open `docs/FEATURE_PARITY_MATRIX.md`
2. Review all features
3. Update statuses (📋 → 🚧 → ✅)
4. Recalculate percentages
5. Update roadmap section
6. Commit changes

### Review API Changelog

1. Move "Unreleased" items to versioned section
2. Tag new API version if breaking changes
3. Update compatibility matrix
4. Communicate changes to teams

### Generate API Comparison Report

```bash
# Script to generate monthly comparison
cd /Users/jamesnorth/sources/eiq-manager

# Generate report
cat > monthly-api-report.md << EOF
# API Report - $(date +%Y-%m)

## API Endpoints
$(php bin/console debug:router | grep api/v1 | wc -l) total endpoints

## Recent Changes
$(git log --since="1 month ago" --oneline src/Controller/Api/)

## Desktop Parity
See FEATURE_PARITY_MATRIX.md

## Action Items
- [ ] Review new endpoints
- [ ] Update desktop types
- [ ] Test compatibility
EOF
```

---

## Troubleshooting

### OpenAPI spec not generating?

```bash
# Clear cache
php bin/console cache:clear

# Check routes
php bin/console debug:router | grep api

# Verify bundle is installed
composer show nelmio/api-doc-bundle
```

### Types not updating?

```bash
# Verify OpenAPI spec is accessible
curl http://localhost:8547/api/openapi.json

# Regenerate types
npm run types:generate

# Check for errors in console
```

### API version headers not appearing?

Check that controllers extend `BaseApiController` and use `apiResponse()` method.

---

## Quick Reference

### Generate Types
```bash
npm run types:generate         # From localhost
npm run types:generate:prod    # From production
```

### Generate API Docs
```bash
php bin/console nelmio:apidoc:dump --format=json > public/api/openapi.json
```

### View API Docs
```
http://localhost:8547/api/doc
```

### Check API Version
```bash
curl -I http://localhost:8547/api/v1/emails | grep X-API-Version
```

---

## Success Criteria

After implementing these steps, you should have:

- ✅ OpenAPI spec auto-generated from code
- ✅ TypeScript types synced from API
- ✅ API version headers on all responses
- ✅ Automated change detection
- ✅ Desktop team notifications
- ✅ Feature parity tracking
- ✅ Clear documentation

---

**Last Updated**: 2025-01-22
**Next Review**: 2025-02-01
