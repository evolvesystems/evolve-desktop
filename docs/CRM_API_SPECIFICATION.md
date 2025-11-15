# CRM Module API Specification

**Module**: CRM Marketing
**Priority**: CRITICAL (Phase 1 MVP)
**Status**: Ready for Implementation
**Date**: 2025-11-14

---

## 🎯 Overview

The CRM Marketing module API provides contact, company, deal, and activity management for the EIQ Desktop application. This specification defines the REST endpoints needed for the desktop client to sync CRM data.

---

## 🔗 API Endpoints

### Base URL
```
/api/v1/crm/
```

---

## 👥 Contacts API

### 1. List Contacts (Paginated)

```http
GET /api/v1/crm/contacts?page=1&limit=50&search=john&tags=customer

Authorization: Bearer {jwt_token}
```

**Query Parameters**:
- `page` (optional) - Page number (default: 1)
- `limit` (optional) - Items per page (default: 50, max: 100)
- `search` (optional) - Search by name, email, phone
- `tags` (optional) - Filter by tags (comma-separated)
- `company_id` (optional) - Filter by company
- `lead_score_min` (optional) - Minimum lead score
- `updated_since` (optional) - ISO datetime for incremental sync

**Response** (200 OK):
```json
{
  "contacts": [
    {
      "id": 123,
      "first_name": "John",
      "last_name": "Doe",
      "email": "john@example.com",
      "phone": "+1-555-0123",
      "company": "Acme Corp",
      "company_id": 45,
      "title": "CEO",
      "lead_score": 85,
      "tags": ["customer", "vip"],
      "custom_fields": {
        "industry": "Technology",
        "employees": "50-100"
      },
      "social_profiles": {
        "linkedin": "https://linkedin.com/in/johndoe",
        "twitter": "@johndoe"
      },
      "address": {
        "street": "123 Main St",
        "city": "San Francisco",
        "state": "CA",
        "postal_code": "94105",
        "country": "USA"
      },
      "notes": "Interested in enterprise plan",
      "last_contacted_at": "2025-11-10T14:30:00Z",
      "created_at": "2025-01-15T10:00:00Z",
      "updated_at": "2025-11-10T14:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 523,
    "has_more": true
  }
}
```

---

### 2. Get Contact by ID

```http
GET /api/v1/crm/contacts/{id}

Authorization: Bearer {jwt_token}
```

**Response** (200 OK):
```json
{
  "id": 123,
  "first_name": "John",
  "last_name": "Doe",
  "email": "john@example.com",
  "phone": "+1-555-0123",
  "company": "Acme Corp",
  "company_id": 45,
  "title": "CEO",
  "lead_score": 85,
  "tags": ["customer", "vip"],
  "custom_fields": {
    "industry": "Technology",
    "employees": "50-100"
  },
  "deals": [
    {
      "id": 67,
      "title": "Enterprise License - Q4 2025",
      "value": 50000.00,
      "stage": "proposal",
      "probability": 75
    }
  ],
  "activities": [
    {
      "id": 89,
      "type": "call",
      "subject": "Follow-up call",
      "completed": true,
      "completed_at": "2025-11-10T14:30:00Z"
    }
  ],
  "timeline": [
    {
      "event_type": "email_sent",
      "description": "Sent proposal email",
      "occurred_at": "2025-11-09T16:00:00Z"
    },
    {
      "event_type": "contact_created",
      "description": "Contact added from LinkedIn",
      "occurred_at": "2025-01-15T10:00:00Z"
    }
  ],
  "created_at": "2025-01-15T10:00:00Z",
  "updated_at": "2025-11-10T14:30:00Z"
}
```

---

### 3. Create Contact

```http
POST /api/v1/crm/contacts

Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "first_name": "Jane",
  "last_name": "Smith",
  "email": "jane@example.com",
  "phone": "+1-555-0456",
  "company_id": 45,
  "title": "CTO",
  "tags": ["lead", "technical"],
  "custom_fields": {
    "industry": "SaaS",
    "budget": "$50k-100k"
  },
  "notes": "Met at conference"
}
```

**Response** (201 Created):
```json
{
  "id": 124,
  "first_name": "Jane",
  "last_name": "Smith",
  "email": "jane@example.com",
  "phone": "+1-555-0456",
  "company_id": 45,
  "title": "CTO",
  "lead_score": 0,
  "tags": ["lead", "technical"],
  "custom_fields": {
    "industry": "SaaS",
    "budget": "$50k-100k"
  },
  "notes": "Met at conference",
  "created_at": "2025-11-14T16:00:00Z",
  "updated_at": "2025-11-14T16:00:00Z"
}
```

---

### 4. Update Contact

```http
PUT /api/v1/crm/contacts/{id}

Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "first_name": "Jane",
  "last_name": "Smith-Johnson",
  "phone": "+1-555-9999",
  "lead_score": 90,
  "tags": ["customer", "technical", "vip"],
  "notes": "Signed contract!"
}
```

**Response** (200 OK):
```json
{
  "id": 124,
  "first_name": "Jane",
  "last_name": "Smith-Johnson",
  "email": "jane@example.com",
  "phone": "+1-555-9999",
  "lead_score": 90,
  "tags": ["customer", "technical", "vip"],
  "updated_at": "2025-11-14T17:00:00Z"
}
```

---

### 5. Delete Contact

```http
DELETE /api/v1/crm/contacts/{id}

Authorization: Bearer {jwt_token}
```

**Response** (200 OK):
```json
{
  "status": "deleted",
  "deleted_at": "2025-11-14T17:30:00Z"
}
```

---

### 6. Bulk Operations

```http
POST /api/v1/crm/contacts/bulk

Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "action": "update_tags", // or "delete", "update_field"
  "contact_ids": [123, 124, 125],
  "data": {
    "tags": ["important", "q4-campaign"]
  }
}
```

**Response** (200 OK):
```json
{
  "processed": 3,
  "failed": 0,
  "results": [
    { "id": 123, "status": "success" },
    { "id": 124, "status": "success" },
    { "id": 125, "status": "success" }
  ]
}
```

---

## 🏢 Companies API

### 1. List Companies

```http
GET /api/v1/crm/companies?page=1&limit=50&search=acme

Authorization: Bearer {jwt_token}
```

**Response** (200 OK):
```json
{
  "companies": [
    {
      "id": 45,
      "name": "Acme Corp",
      "website": "https://acme.com",
      "industry": "Technology",
      "employee_count": 250,
      "annual_revenue": 5000000.00,
      "address": {
        "street": "456 Business Blvd",
        "city": "New York",
        "state": "NY",
        "postal_code": "10001",
        "country": "USA"
      },
      "contact_count": 12,
      "deal_count": 3,
      "created_at": "2024-06-01T09:00:00Z",
      "updated_at": "2025-11-10T12:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 87,
    "has_more": true
  }
}
```

---

### 2. Create Company

```http
POST /api/v1/crm/companies

Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "name": "TechStart Inc",
  "website": "https://techstart.io",
  "industry": "SaaS",
  "employee_count": 50,
  "annual_revenue": 2000000.00
}
```

**Response** (201 Created):
```json
{
  "id": 88,
  "name": "TechStart Inc",
  "website": "https://techstart.io",
  "industry": "SaaS",
  "employee_count": 50,
  "annual_revenue": 2000000.00,
  "contact_count": 0,
  "deal_count": 0,
  "created_at": "2025-11-14T16:30:00Z",
  "updated_at": "2025-11-14T16:30:00Z"
}
```

---

## 💼 Deals API

### 1. List Deals (Pipeline View)

```http
GET /api/v1/crm/deals?stage=proposal&sort=value_desc

Authorization: Bearer {jwt_token}
```

**Query Parameters**:
- `stage` (optional) - Filter by stage: lead, qualified, proposal, negotiation, won, lost
- `contact_id` (optional) - Filter by contact
- `company_id` (optional) - Filter by company
- `min_value`, `max_value` (optional) - Value range filter
- `sort` (optional) - value_desc, value_asc, close_date, created_at

**Response** (200 OK):
```json
{
  "deals": [
    {
      "id": 67,
      "title": "Enterprise License - Q4 2025",
      "value": 50000.00,
      "currency": "USD",
      "stage": "proposal",
      "probability": 75,
      "contact_id": 123,
      "contact_name": "John Doe",
      "company_id": 45,
      "company_name": "Acme Corp",
      "expected_close_date": "2025-12-15",
      "owner_id": 5,
      "owner_name": "Alice Sales",
      "notes": "Waiting for procurement approval",
      "created_at": "2025-10-01T10:00:00Z",
      "updated_at": "2025-11-10T15:00:00Z"
    }
  ],
  "summary": {
    "total_value": 250000.00,
    "total_count": 8,
    "by_stage": {
      "lead": { "count": 2, "value": 30000.00 },
      "qualified": { "count": 1, "value": 20000.00 },
      "proposal": { "count": 3, "value": 150000.00 },
      "negotiation": { "count": 2, "value": 50000.00 }
    }
  }
}
```

---

### 2. Create Deal

```http
POST /api/v1/crm/deals

Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "title": "Professional Plan - TechStart",
  "value": 15000.00,
  "currency": "USD",
  "stage": "qualified",
  "probability": 60,
  "contact_id": 124,
  "company_id": 88,
  "expected_close_date": "2025-12-31",
  "notes": "Interested in annual subscription"
}
```

**Response** (201 Created):
```json
{
  "id": 68,
  "title": "Professional Plan - TechStart",
  "value": 15000.00,
  "currency": "USD",
  "stage": "qualified",
  "probability": 60,
  "contact_id": 124,
  "company_id": 88,
  "expected_close_date": "2025-12-31",
  "notes": "Interested in annual subscription",
  "created_at": "2025-11-14T16:45:00Z",
  "updated_at": "2025-11-14T16:45:00Z"
}
```

---

### 3. Update Deal Stage

```http
PUT /api/v1/crm/deals/{id}/stage

Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "stage": "won",
  "probability": 100,
  "closed_date": "2025-11-14",
  "notes": "Contract signed!"
}
```

**Response** (200 OK):
```json
{
  "id": 68,
  "stage": "won",
  "probability": 100,
  "closed_date": "2025-11-14",
  "updated_at": "2025-11-14T17:00:00Z"
}
```

---

## 📝 Activities API

### 1. List Activities

```http
GET /api/v1/crm/activities?contact_id=123&completed=false

Authorization: Bearer {jwt_token}
```

**Query Parameters**:
- `contact_id` (optional) - Filter by contact
- `deal_id` (optional) - Filter by deal
- `type` (optional) - call, email, meeting, task, note
- `completed` (optional) - true, false
- `due_before` (optional) - ISO datetime

**Response** (200 OK):
```json
{
  "activities": [
    {
      "id": 89,
      "type": "call",
      "subject": "Follow-up on proposal",
      "description": "Discuss pricing and timeline",
      "contact_id": 123,
      "contact_name": "John Doe",
      "deal_id": 67,
      "deal_title": "Enterprise License - Q4 2025",
      "completed": false,
      "due_date": "2025-11-15T14:00:00Z",
      "reminder_at": "2025-11-15T13:45:00Z",
      "assigned_to_id": 5,
      "assigned_to_name": "Alice Sales",
      "created_at": "2025-11-14T10:00:00Z",
      "updated_at": "2025-11-14T10:00:00Z"
    }
  ]
}
```

---

### 2. Create Activity

```http
POST /api/v1/crm/activities

Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "type": "meeting",
  "subject": "Demo presentation",
  "description": "Product demo for TechStart team",
  "contact_id": 124,
  "deal_id": 68,
  "due_date": "2025-11-20T10:00:00Z",
  "duration_minutes": 60,
  "reminder_at": "2025-11-20T09:45:00Z"
}
```

**Response** (201 Created):
```json
{
  "id": 90,
  "type": "meeting",
  "subject": "Demo presentation",
  "description": "Product demo for TechStart team",
  "contact_id": 124,
  "deal_id": 68,
  "due_date": "2025-11-20T10:00:00Z",
  "duration_minutes": 60,
  "reminder_at": "2025-11-20T09:45:00Z",
  "completed": false,
  "created_at": "2025-11-14T17:15:00Z"
}
```

---

### 3. Mark Activity Complete

```http
PUT /api/v1/crm/activities/{id}/complete

Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "outcome_notes": "Great conversation, moving to proposal stage"
}
```

**Response** (200 OK):
```json
{
  "id": 90,
  "completed": true,
  "completed_at": "2025-11-14T17:30:00Z",
  "outcome_notes": "Great conversation, moving to proposal stage"
}
```

---

## 🔍 Search API

### Global CRM Search

```http
GET /api/v1/crm/search?q=acme&types=contacts,companies,deals

Authorization: Bearer {jwt_token}
```

**Query Parameters**:
- `q` (required) - Search query
- `types` (optional) - Comma-separated: contacts, companies, deals, activities

**Response** (200 OK):
```json
{
  "results": {
    "contacts": [
      {
        "id": 123,
        "type": "contact",
        "first_name": "John",
        "last_name": "Doe",
        "email": "john@acme.com",
        "company": "Acme Corp",
        "relevance": 0.95
      }
    ],
    "companies": [
      {
        "id": 45,
        "type": "company",
        "name": "Acme Corp",
        "website": "https://acme.com",
        "contact_count": 12,
        "relevance": 1.0
      }
    ],
    "deals": [
      {
        "id": 67,
        "type": "deal",
        "title": "Enterprise License - Acme",
        "value": 50000.00,
        "stage": "proposal",
        "relevance": 0.88
      }
    ]
  },
  "total_results": 3
}
```

---

## 📊 Analytics API

### CRM Dashboard Stats

```http
GET /api/v1/crm/analytics/dashboard

Authorization: Bearer {jwt_token}
```

**Response** (200 OK):
```json
{
  "contacts": {
    "total": 523,
    "added_this_month": 45,
    "by_source": {
      "manual": 200,
      "import": 150,
      "email": 100,
      "linkedin": 73
    },
    "top_tags": ["customer", "lead", "vip", "partner"]
  },
  "deals": {
    "total_value": 1250000.00,
    "total_count": 23,
    "won_this_month": 3,
    "won_value_this_month": 85000.00,
    "pipeline_by_stage": {
      "lead": { "count": 5, "value": 75000.00 },
      "qualified": { "count": 4, "value": 120000.00 },
      "proposal": { "count": 8, "value": 450000.00 },
      "negotiation": { "count": 3, "value": 200000.00 },
      "won": { "count": 3, "value": 85000.00 }
    }
  },
  "activities": {
    "upcoming": 12,
    "overdue": 3,
    "completed_this_week": 28
  }
}
```

---

## 🔄 Sync API

### Incremental Sync

```http
POST /api/v1/crm/sync

Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "last_sync": "2025-11-14T12:00:00Z",
  "modules": ["contacts", "companies", "deals", "activities"]
}
```

**Response** (200 OK):
```json
{
  "sync_id": "sync_abc123",
  "timestamp": "2025-11-14T18:00:00Z",
  "changes": {
    "contacts": {
      "created": 5,
      "updated": 12,
      "deleted": 2,
      "items": [
        {
          "id": 125,
          "action": "created",
          "data": { /* full contact object */ }
        },
        {
          "id": 123,
          "action": "updated",
          "data": { /* full contact object */ }
        },
        {
          "id": 100,
          "action": "deleted",
          "deleted_at": "2025-11-14T17:00:00Z"
        }
      ]
    },
    "companies": {
      "created": 1,
      "updated": 3,
      "deleted": 0,
      "items": [ /* ... */ ]
    },
    "deals": {
      "created": 2,
      "updated": 5,
      "deleted": 1,
      "items": [ /* ... */ ]
    },
    "activities": {
      "created": 8,
      "updated": 15,
      "deleted": 10,
      "items": [ /* ... */ ]
    }
  },
  "has_more": false
}
```

---

## 🚨 Error Responses

### Standard Error Format

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Email is required",
    "details": {
      "field": "email",
      "constraint": "required"
    }
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `UNAUTHORIZED` | 401 | Invalid or missing JWT token |
| `FORBIDDEN` | 403 | User lacks permission for resource |
| `NOT_FOUND` | 404 | Resource doesn't exist |
| `VALIDATION_ERROR` | 422 | Invalid input data |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests |
| `SERVER_ERROR` | 500 | Internal server error |

---

## 📋 Implementation Checklist

### Backend (Symfony - eiq-manager)

- [ ] Create `/api/v1/crm/` route group
- [ ] Implement `CrmContactApiController`
  - [ ] `GET /contacts` - List with pagination
  - [ ] `GET /contacts/{id}` - Single contact
  - [ ] `POST /contacts` - Create
  - [ ] `PUT /contacts/{id}` - Update
  - [ ] `DELETE /contacts/{id}` - Delete
  - [ ] `POST /contacts/bulk` - Bulk operations
- [ ] Implement `CrmCompanyApiController`
  - [ ] `GET /companies` - List
  - [ ] `POST /companies` - Create
  - [ ] `PUT /companies/{id}` - Update
- [ ] Implement `CrmDealApiController`
  - [ ] `GET /deals` - List with pipeline view
  - [ ] `POST /deals` - Create
  - [ ] `PUT /deals/{id}/stage` - Update stage
- [ ] Implement `CrmActivityApiController`
  - [ ] `GET /activities` - List
  - [ ] `POST /activities` - Create
  - [ ] `PUT /activities/{id}/complete` - Mark complete
- [ ] Implement `CrmSearchApiController`
  - [ ] `GET /search` - Global search
- [ ] Implement `CrmSyncApiController`
  - [ ] `POST /sync` - Incremental sync
- [ ] Add validation rules for all endpoints
- [ ] Add unit tests for controllers
- [ ] Add integration tests for sync logic

### Desktop (Tauri - EIQ Desktop)

- [ ] Create CRM module plugin
- [ ] Implement SQLite schema for CRM tables
- [ ] Create API client service (`services/crm.service.ts`)
- [ ] Build Vue components:
  - [ ] `ContactList.vue` - Contact list view
  - [ ] `ContactDetail.vue` - Contact details
  - [ ] `ContactEditor.vue` - Create/edit form
  - [ ] `CompanyList.vue` - Company list
  - [ ] `DealPipeline.vue` - Kanban board
  - [ ] `ActivityList.vue` - Task list
- [ ] Implement offline sync logic
- [ ] Add search functionality
- [ ] Create notification handlers
- [ ] Write unit tests for CRM module

---

**API Version**: 1.0
**Last Updated**: 2025-11-14
**Status**: Ready for Implementation
