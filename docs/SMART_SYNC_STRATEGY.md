# Smart EIQ Manager ↔ Desktop App Synchronization Strategy

## Overview

This document outlines the comprehensive strategy for keeping the EvolveApp Desktop client synchronized with the constantly evolving EIQ Manager platform.

**Challenge**: EIQ Manager modules are constantly being improved with new features, UI changes, and API updates. The desktop app needs to stay in sync without manual tracking becoming a bottleneck.

**Solution**: Automated synchronization using API versioning, type generation, shared components, and feature parity tracking.

---

## 1. API Versioning Infrastructure

### Current State
- API endpoints use `/api/v1/` prefix
- No formal versioning strategy
- Changes tracked only via git commits

### Proposed Solution

#### A. Semantic Versioning for API

**Version Format**: `MAJOR.MINOR.PATCH`

- **MAJOR** (2.0.0) - Breaking changes that require desktop app updates
- **MINOR** (1.1.0) - New features, backward compatible
- **PATCH** (1.0.1) - Bug fixes, no API changes

**Implementation**:
```php
// config/routes.yaml
api_v1:
    resource: '../src/Controller/Api/'
    type: attribute
    prefix: /api/v1

api_v2:  # For future breaking changes
    resource: '../src/Controller/Api/V2/'
    type: attribute
    prefix: /api/v2
```

#### B. API Response Headers

Every API response includes version information:
```php
// BaseApiController.php
protected function jsonResponse($data, int $status = 200): JsonResponse
{
    return $this->json($data, $status, [
        'X-API-Version' => '1.0.0',
        'X-API-Deprecation-Warning' => null, // Or deprecation message
        'X-API-Sunset-Date' => null // When endpoint will be removed
    ]);
}
```

#### C. Deprecation Strategy

When changing API:
1. Add new endpoint/field (v1.1.0)
2. Mark old endpoint/field as deprecated with sunset date
3. Desktop app updates to use new endpoint
4. Remove old endpoint in next major version (v2.0.0)

**Example**:
```php
/**
 * @deprecated This endpoint is deprecated and will be removed in v2.0.0
 * @sunset 2025-06-01
 * Use /api/v1/emails/messages instead
 */
#[Route('/api/v1/emails/list', name: 'api_email_list_old')]
public function listOld(): JsonResponse
{
    return $this->jsonResponse([
        'warning' => 'This endpoint is deprecated. Use /api/v1/emails/messages'
    ]);
}
```

---

## 2. OpenAPI/Swagger Specification

### Implementation

#### A. Install nelmio/api-doc-bundle

```bash
cd /Users/jamesnorth/sources/eiq-manager
composer require nelmio/api-doc-bundle
```

#### B. Configure Bundle

```yaml
# config/packages/nelmio_api_doc.yaml
nelmio_api_doc:
    documentation:
        info:
            title: EIQ Manager API
            description: REST API for EvolveApp Desktop and Mobile clients
            version: 1.0.0
        servers:
            - url: http://localhost:8547
              description: Local development
            - url: https://evolvepreneuriq.app
              description: Production
        components:
            securitySchemes:
                bearerAuth:
                    type: http
                    scheme: bearer
                    bearerFormat: JWT
    areas:
        path_patterns:
            - ^/api(?!/doc$)
```

#### C. Annotate Controllers

```php
use OpenApi\Attributes as OA;

#[OA\Get(
    path: '/api/v1/emails',
    summary: 'List emails with filtering and pagination',
    tags: ['Email'],
    parameters: [
        new OA\Parameter(
            name: 'page',
            in: 'query',
            description: 'Page number',
            schema: new OA\Schema(type: 'integer', default: 1)
        ),
        new OA\Parameter(
            name: 'limit',
            in: 'query',
            description: 'Items per page',
            schema: new OA\Schema(type: 'integer', default: 50, maximum: 100)
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
                        items: new OA\Items(ref: '#/components/schemas/Email')
                    ),
                    new OA\Property(
                        property: 'meta',
                        ref: '#/components/schemas/PaginationMeta'
                    )
                ]
            )
        )
    ],
    security: [['bearerAuth' => []]]
)]
#[Route('/api/v1/emails', name: 'api_email_list', methods: ['GET'])]
public function list(Request $request): JsonResponse
```

#### D. Generate Specification

```bash
# Generate OpenAPI spec as JSON
php bin/console nelmio:apidoc:dump --format=json > public/api/openapi.json

# Generate as YAML
php bin/console nelmio:apidoc:dump --format=yaml > public/api/openapi.yaml
```

#### E. Automated Generation

Add to CI/CD pipeline:
```yaml
# .github/workflows/api-docs.yml
name: Generate API Docs

on:
  push:
    branches: [master]
    paths:
      - 'src/Controller/Api/**'

jobs:
  generate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install PHP
        uses: shivammathur/setup-php@v2
        with:
          php-version: '8.2'

      - name: Install dependencies
        run: composer install

      - name: Generate OpenAPI spec
        run: php bin/console nelmio:apidoc:dump --format=json > public/api/openapi.json

      - name: Commit changes
        run: |
          git config --global user.name 'API Docs Bot'
          git config --global user.email 'bot@evolvepreneuriq.app'
          git add public/api/openapi.json
          git commit -m "chore: update OpenAPI spec" || exit 0
          git push
```

---

## 3. TypeScript Type Generation

### A. Install TypeScript Generator

In desktop app:
```bash
cd /Users/jamesnorth/sources/evolveapp/evolve-desktop
npm install --save-dev openapi-typescript
```

### B. Generate Types Script

```json
// package.json
{
  "scripts": {
    "types:generate": "openapi-typescript http://localhost:8547/api/openapi.json -o src/types/api.ts",
    "types:generate:prod": "openapi-typescript https://evolvepreneuriq.app/api/openapi.json -o src/types/api.ts"
  }
}
```

### C. Generated Types Structure

```typescript
// src/types/api.ts (auto-generated)
export interface paths {
  "/api/v1/emails": {
    get: {
      parameters: {
        query: {
          page?: number;
          limit?: number;
          folder_id?: number;
          search?: string;
        };
      };
      responses: {
        200: {
          content: {
            "application/json": {
              data: EmailMessage[];
              meta: PaginationMeta;
            };
          };
        };
      };
    };
  };
}

export interface components {
  schemas: {
    EmailMessage: {
      id: number;
      subject: string;
      from_address: string;
      to_address: string;
      body_html?: string;
      body_text?: string;
      received_date: string;
      is_read: boolean;
      is_flagged: boolean;
      has_attachments: boolean;
    };
    PaginationMeta: {
      page: number;
      limit: number;
      total: number;
      pages: number;
    };
  };
}
```

### D. Type-Safe API Client

```typescript
// src/services/api/client.ts
import type { paths, components } from '@/types/api';
import axios, { AxiosInstance } from 'axios';

type EmailMessage = components['schemas']['EmailMessage'];
type ListEmailsParams = paths['/api/v1/emails']['get']['parameters']['query'];
type ListEmailsResponse = paths['/api/v1/emails']['get']['responses']['200']['content']['application/json'];

class ApiClient {
  private client: AxiosInstance;

  constructor(baseURL: string) {
    this.client = axios.create({ baseURL });
  }

  // Type-safe email listing
  async listEmails(params: ListEmailsParams): Promise<ListEmailsResponse> {
    const response = await this.client.get<ListEmailsResponse>('/api/v1/emails', {
      params
    });
    return response.data;
  }

  // Compile-time error if API changes
  async getEmail(id: number): Promise<EmailMessage> {
    const response = await this.client.get<EmailMessage>(`/api/v1/emails/${id}`);
    return response.data;
  }
}

export const apiClient = new ApiClient(
  import.meta.env.VITE_API_URL || 'http://localhost:8547'
);
```

### E. Automated Type Updates

```yaml
# .github/workflows/sync-types.yml
name: Sync API Types

on:
  repository_dispatch:
    types: [api-updated]
  schedule:
    - cron: '0 0 * * *'  # Daily at midnight

jobs:
  sync:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'

      - name: Install dependencies
        run: cd evolve-desktop && npm install

      - name: Generate types from production API
        run: cd evolve-desktop && npm run types:generate:prod

      - name: Create Pull Request
        uses: peter-evans/create-pull-request@v5
        with:
          title: 'chore: update API types from EIQ Manager'
          body: |
            Auto-generated PR to sync API types.

            Review changes carefully - may require code updates.
          branch: auto/sync-api-types
          commit-message: 'chore: sync API types from EIQ Manager'
```

---

## 4. Shared UI Components Library

### A. Create Shared Package

```bash
mkdir -p /Users/jamesnorth/sources/evolveiq-ui-components
cd /Users/jamesnorth/sources/evolveiq-ui-components

npm init -y
```

```json
// package.json
{
  "name": "@evolveiq/ui-components",
  "version": "1.0.0",
  "type": "module",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "import": "./dist/index.js",
      "types": "./dist/index.d.ts"
    },
    "./styles": "./dist/style.css"
  },
  "files": ["dist"],
  "scripts": {
    "build": "vite build && vue-tsc --declaration --emitDeclarationOnly",
    "dev": "vite",
    "storybook": "storybook dev -p 6006"
  },
  "peerDependencies": {
    "vue": "^3.5.0"
  },
  "dependencies": {
    "daisyui": "^5.5.3",
    "tailwindcss": "^3.4.18"
  }
}
```

### B. Component Structure

```
evolveiq-ui-components/
├── src/
│   ├── components/
│   │   ├── Button/
│   │   │   ├── Button.vue
│   │   │   ├── Button.stories.ts
│   │   │   └── Button.test.ts
│   │   ├── Card/
│   │   ├── Modal/
│   │   ├── EmailList/
│   │   ├── ContactCard/
│   │   └── ...
│   ├── composables/
│   │   ├── useTheme.ts
│   │   └── usePagination.ts
│   ├── index.ts
│   └── style.css
├── package.json
├── vite.config.ts
└── tsconfig.json
```

### C. Shared Component Example

```vue
<!-- src/components/EmailList/EmailList.vue -->
<script setup lang="ts">
import type { EmailMessage } from '@/types';

interface Props {
  emails: EmailMessage[];
  loading?: boolean;
  onSelect?: (email: EmailMessage) => void;
}

const props = defineProps<Props>();

function handleSelect(email: EmailMessage) {
  props.onSelect?.(email);
}
</script>

<template>
  <div class="email-list">
    <div v-if="loading" class="loading loading-spinner"></div>

    <div
      v-for="email in emails"
      :key="email.id"
      class="email-item"
      @click="handleSelect(email)"
    >
      <div class="email-from">{{ email.from_address }}</div>
      <div class="email-subject">{{ email.subject }}</div>
      <div class="email-date">{{ email.received_date }}</div>
    </div>
  </div>
</template>

<style scoped>
/* Shared styles using Tailwind/DaisyUI */
.email-list {
  @apply divide-y divide-base-300;
}

.email-item {
  @apply p-4 hover:bg-base-200 cursor-pointer transition;
}
</style>
```

### D. Using Shared Components

**In Desktop App**:
```bash
cd /Users/jamesnorth/sources/evolveapp/evolve-desktop
npm install @evolveiq/ui-components
```

```vue
<!-- src/modules/email/views/EmailView.vue -->
<script setup lang="ts">
import { EmailList } from '@evolveiq/ui-components';
import { ref } from 'vue';

const emails = ref([]);
</script>

<template>
  <EmailList :emails="emails" @select="handleEmailSelect" />
</template>
```

---

## 5. Feature Parity Tracking System

### A. Feature Matrix Dashboard

Create a web dashboard showing what's implemented:

```typescript
// feature-parity-tracker/index.html
interface ModuleFeature {
  module: string;
  feature: string;
  webStatus: 'implemented' | 'in-progress' | 'planned';
  desktopStatus: 'implemented' | 'in-progress' | 'planned' | 'not-applicable';
  apiVersion: string;
  priority: 'high' | 'medium' | 'low';
}

const features: ModuleFeature[] = [
  {
    module: 'Email',
    feature: 'Read emails',
    webStatus: 'implemented',
    desktopStatus: 'implemented',
    apiVersion: '1.0.0',
    priority: 'high'
  },
  {
    module: 'Email',
    feature: 'Rich text compose',
    webStatus: 'implemented',
    desktopStatus: 'in-progress',
    apiVersion: '1.0.0',
    priority: 'high'
  },
  {
    module: 'CRM',
    feature: 'Contact management',
    webStatus: 'implemented',
    desktopStatus: 'planned',
    apiVersion: '1.1.0',
    priority: 'medium'
  }
  // ... more features
];
```

### B. Automated Feature Detection

Script to scan code and detect implemented features:

```javascript
// scripts/detect-features.js
const fs = require('fs');
const path = require('path');

// Scan EIQ Manager for implemented features
function scanBackendFeatures() {
  const controllers = findFiles('src/Controller/Api', '.php');
  const features = [];

  controllers.forEach(file => {
    const content = fs.readFileSync(file, 'utf8');
    const routes = extractRoutes(content);

    routes.forEach(route => {
      features.push({
        endpoint: route.path,
        method: route.method,
        controller: path.basename(file),
        status: 'implemented'
      });
    });
  });

  return features;
}

// Scan Desktop App for implemented features
function scanDesktopFeatures() {
  const services = findFiles('src/services', '.ts');
  const features = [];

  services.forEach(file => {
    const content = fs.readFileSync(file, 'utf8');
    const apiCalls = extractApiCalls(content);

    apiCalls.forEach(call => {
      features.push({
        endpoint: call.endpoint,
        component: path.basename(file),
        status: 'implemented'
      });
    });
  });

  return features;
}

// Compare and generate parity report
function generateParityReport() {
  const backendFeatures = scanBackendFeatures();
  const desktopFeatures = scanDesktopFeatures();

  const report = {
    totalBackendEndpoints: backendFeatures.length,
    totalDesktopImplemented: desktopFeatures.length,
    parity: (desktopFeatures.length / backendFeatures.length * 100).toFixed(2) + '%',
    missing: backendFeatures.filter(
      bf => !desktopFeatures.find(df => df.endpoint === bf.endpoint)
    )
  };

  fs.writeFileSync('feature-parity-report.json', JSON.stringify(report, null, 2));
}

generateParityReport();
```

### C. Dashboard

```html
<!-- docs/feature-parity/index.html -->
<!DOCTYPE html>
<html>
<head>
  <title>EIQ Manager ↔ Desktop Feature Parity</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <link href="https://cdn.jsdelivr.net/npm/daisyui@5.5.3/dist/full.css" rel="stylesheet">
</head>
<body>
  <div class="container mx-auto p-8">
    <h1 class="text-4xl font-bold mb-8">Feature Parity Dashboard</h1>

    <div class="stats shadow mb-8">
      <div class="stat">
        <div class="stat-title">Total Features (Web)</div>
        <div class="stat-value" id="total-web">152</div>
      </div>
      <div class="stat">
        <div class="stat-title">Implemented (Desktop)</div>
        <div class="stat-value text-primary" id="total-desktop">87</div>
      </div>
      <div class="stat">
        <div class="stat-title">Parity</div>
        <div class="stat-value text-secondary" id="parity">57%</div>
      </div>
    </div>

    <table class="table table-zebra w-full">
      <thead>
        <tr>
          <th>Module</th>
          <th>Feature</th>
          <th>Web Status</th>
          <th>Desktop Status</th>
          <th>API Version</th>
          <th>Priority</th>
        </tr>
      </thead>
      <tbody id="features-table">
        <!-- Populated by JavaScript -->
      </tbody>
    </table>
  </div>

  <script src="./features.js"></script>
</body>
</html>
```

---

## 6. Change Detection & Notification

### A. Weekly API Diff Script

```bash
#!/bin/bash
# scripts/api-diff.sh

# Fetch latest OpenAPI spec from production
curl -s https://evolvepreneuriq.app/api/openapi.json > /tmp/openapi-new.json

# Compare with last week's version
if [ -f api-snapshots/openapi-$(date -d '7 days ago' +%Y-%m-%d).json ]; then
  diff -u \
    api-snapshots/openapi-$(date -d '7 days ago' +%Y-%m-%d).json \
    /tmp/openapi-new.json \
    > /tmp/api-diff.txt

  # If there are differences, create GitHub issue
  if [ -s /tmp/api-diff.txt ]; then
    gh issue create \
      --title "API Changes Detected ($(date +%Y-%m-%d))" \
      --body "$(cat /tmp/api-diff.txt)" \
      --label "api-change" \
      --assignee "desktop-team"
  fi
fi

# Save snapshot
cp /tmp/openapi-new.json api-snapshots/openapi-$(date +%Y-%m-%d).json
```

### B. GitHub Actions for Change Detection

```yaml
# .github/workflows/api-change-detection.yml
name: API Change Detection

on:
  schedule:
    - cron: '0 9 * * 1'  # Every Monday at 9 AM
  workflow_dispatch:

jobs:
  detect:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Fetch latest API spec
        run: |
          curl -s https://evolvepreneuriq.app/api/openapi.json > openapi-new.json

      - name: Compare with previous version
        id: diff
        run: |
          if [ -f api-snapshots/openapi-latest.json ]; then
            diff -u api-snapshots/openapi-latest.json openapi-new.json > diff.txt || true
            echo "has_changes=$([ -s diff.txt ] && echo 'true' || echo 'false')" >> $GITHUB_OUTPUT
          fi

      - name: Create issue if changes detected
        if: steps.diff.outputs.has_changes == 'true'
        uses: actions/github-script@v6
        with:
          script: |
            const fs = require('fs');
            const diff = fs.readFileSync('diff.txt', 'utf8');

            github.rest.issues.create({
              owner: context.repo.owner,
              repo: context.repo.repo,
              title: `API Changes Detected - ${new Date().toISOString().split('T')[0]}`,
              body: `## API Changes Detected\n\n\`\`\`diff\n${diff}\n\`\`\`\n\n**Action Required**: Review changes and update desktop app accordingly.`,
              labels: ['api-change', 'needs-review']
            });

      - name: Save snapshot
        run: |
          mkdir -p api-snapshots
          cp openapi-new.json api-snapshots/openapi-latest.json

      - name: Commit snapshot
        run: |
          git config --global user.name 'API Monitor Bot'
          git config --global user.email 'bot@evolvepreneuriq.app'
          git add api-snapshots/
          git commit -m "chore: update API snapshot" || exit 0
          git push
```

---

## 7. Development Workflow

### Daily Workflow

**Backend Developer** (EIQ Manager):
1. Make changes to API controller
2. Add OpenAPI annotations
3. Update `docs/API_CHANGELOG.md` (Unreleased section)
4. Run tests
5. Commit & push
6. CI/CD generates new OpenAPI spec
7. Desktop team gets notified via GitHub issue

**Desktop Developer**:
1. Check for new GitHub issues labeled "api-change"
2. Run `npm run types:generate` to get latest types
3. TypeScript compiler shows errors where API changed
4. Update desktop code to match new API
5. Test changes
6. Commit & push

### Weekly Sync Meeting
- Review API changes from past week
- Discuss upcoming breaking changes
- Prioritize desktop features based on web changes
- Update feature parity dashboard

---

## 8. Continuous Integration

### A. Contract Testing

Ensure desktop app and backend stay compatible:

```typescript
// tests/contract/email-api.test.ts
import { apiClient } from '@/services/api/client';
import { describe, it, expect } from 'vitest';

describe('Email API Contract Tests', () => {
  it('should list emails with correct structure', async () => {
    const response = await apiClient.listEmails({ page: 1, limit: 10 });

    // Verify response structure matches TypeScript types
    expect(response).toHaveProperty('data');
    expect(response).toHaveProperty('meta');
    expect(response.meta).toHaveProperty('page');
    expect(response.meta).toHaveProperty('total');

    // Verify each email has required fields
    if (response.data.length > 0) {
      const email = response.data[0];
      expect(email).toHaveProperty('id');
      expect(email).toHaveProperty('subject');
      expect(email).toHaveProperty('from_address');
      expect(email).toHaveProperty('received_date');
    }
  });

  it('should handle authentication correctly', async () => {
    // Test with invalid token
    // Should get 401 Unauthorized
  });
});
```

### B. E2E Tests Against Staging

```yaml
# .github/workflows/e2e-tests.yml
name: E2E Tests

on:
  pull_request:
  schedule:
    - cron: '0 */6 * * *'  # Every 6 hours

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3

      - name: Install dependencies
        run: cd evolve-desktop && npm install

      - name: Run E2E tests against staging
        env:
          VITE_API_URL: https://staging.evolvepreneuriq.app
          TEST_USER_EMAIL: test@example.com
          TEST_USER_PASSWORD: ${{ secrets.TEST_PASSWORD }}
        run: cd evolve-desktop && npm run test:e2e

      - name: Notify on failure
        if: failure()
        uses: actions/github-script@v6
        with:
          script: |
            github.rest.issues.create({
              owner: context.repo.owner,
              repo: context.repo.repo,
              title: 'E2E Tests Failed - API Compatibility Issue',
              body: 'Desktop app E2E tests failed. This may indicate API changes that broke compatibility.',
              labels: ['bug', 'api-compatibility']
            });
```

---

## 9. Implementation Roadmap

### Week 1-2: API Infrastructure
- ✅ Create API_CHANGELOG.md
- [ ] Install nelmio/api-doc-bundle
- [ ] Add OpenAPI annotations to all API controllers
- [ ] Set up automated spec generation
- [ ] Add API version headers

### Week 2-3: Type Generation
- [ ] Set up openapi-typescript in desktop app
- [ ] Create type generation scripts
- [ ] Set up automated PR creation for type updates
- [ ] Migrate existing API calls to use generated types

### Week 3-4: Shared Components
- [ ] Create @evolveiq/ui-components package
- [ ] Migrate common components from desktop app
- [ ] Set up Storybook for component documentation
- [ ] Publish to npm or private registry

### Week 4-5: Feature Tracking
- [ ] Build feature parity dashboard
- [ ] Create automated feature detection scripts
- [ ] Set up weekly API diff checking
- [ ] Implement GitHub issue automation

### Week 5-6: CI/CD Integration
- [ ] Set up contract testing
- [ ] Configure E2E tests against staging
- [ ] Add API compatibility checks to PR pipeline
- [ ] Create release coordination workflow

---

## 10. Benefits Summary

### For Backend Developers
- Clear process for API changes
- Automatic documentation generation
- Desktop team is always informed
- No manual coordination needed

### For Desktop Developers
- Automatic type updates
- Compile-time error detection for API changes
- Shared components reduce duplication
- Clear visibility into what needs implementation

### For Product Team
- Feature parity dashboard shows progress
- Can prioritize desktop features based on web
- API versioning prevents breaking users
- Coordinated release process

### For Users
- Consistent experience across web and desktop
- New features appear on desktop soon after web
- Fewer bugs from API mismatches
- Smooth upgrade path

---

## 11. Maintenance

### Weekly Tasks
- Check API_CHANGELOG for unreleased changes
- Review feature parity dashboard
- Update desktop app types
- Run contract tests

### Monthly Tasks
- Review deprecated endpoints and plan removal
- Update shared component library
- Audit feature parity and create implementation plan
- Performance review of API

### Quarterly Tasks
- Major version planning (if breaking changes needed)
- Desktop app feature roadmap alignment
- Shared component library major updates
- Security audit

---

**Document Version**: 1.0.0
**Last Updated**: 2025-01-22
**Owner**: EIQ Platform Team
**Reviewers**: Desktop Team, Backend Team
