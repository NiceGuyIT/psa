# Professional Services Automation (PSA) Platform
## Product Requirements Document (PRD)

**Version:** 1.0
**Date:** January 2026
**Status:** Draft

---

## 1. Executive Summary

This document outlines the requirements for a Professional Services Automation (PSA) platform designed specifically for Managed Service Providers (MSPs). The platform will be built as a multi-tenant SaaS solution with the capability to deploy as a single-tenant self-hosted instance via compile-time configuration.

### 1.1 Vision Statement

Provide MSPs with a comprehensive, modern, and secure platform to manage their entire service delivery lifecycle—from initial client contact through service delivery, billing, and ongoing relationship management.

### 1.2 Target Users

- **MSP Administrators**: Configure and manage the platform
- **MSP Technicians**: Handle tickets, projects, and time tracking
- **MSP Managers**: Oversee operations, resources, and financials
- **MSP Sales/Account Managers**: Manage client relationships and contracts
- **End Clients**: Access portal for tickets, invoices, and service requests

---

## 2. Architecture Overview

### 2.1 Deployment Modes

| Feature Flag | Mode | Description |
|--------------|------|-------------|
| `multi-tenant` | SaaS | Shared infrastructure, tenant isolation at database level |
| `single-tenant` | Self-Hosted | Dedicated instance for single MSP |

### 2.2 Technology Stack

| Layer | Technology | Version |
|-------|------------|---------|
| Backend Framework | Dioxus (fullstack) | 0.7.2 |
| HTTP Server | Axum | 0.8.8 |
| Database | PostgreSQL | 18 |
| Frontend | Dioxus WASM + SSR | 0.7.2 |
| HTTP Client | gloo-net | 0.6 |
| WASM Bindings | wasm-bindgen | 0.2 |
| Styling | Tailwind CSS | 4 |

### 2.3 Module Architecture

All modules are designed as discrete services with well-defined APIs. Modules communicate via internal API contracts, enabling:

- Independent module development and testing
- Optional module enablement via configuration
- Clear separation of concerns
- Future microservices extraction if needed

```
┌─────────────────────────────────────────────────────────────────────┐
│                         CLIENT LAYER                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │  Web Client  │  │ Client Portal│  │  Mobile App  │              │
│  │   (WASM)     │  │   (WASM)     │  │  (Phase 2)   │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
└─────────────────────────────────────────────────────────────────────┘
                              │ HTTPS/WSS
┌─────────────────────────────────────────────────────────────────────┐
│                         API GATEWAY                                  │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  Authentication │ Rate Limiting │ Tenant Resolution │ CORS  │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────────────┐
│                       MODULE LAYER                                   │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐       │
│  │  Ticketing │ │   Time     │ │  Projects  │ │  Contacts  │       │
│  │   Module   │ │  Tracking  │ │   Module   │ │   Module   │       │
│  └────────────┘ └────────────┘ └────────────┘ └────────────┘       │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐       │
│  │  Calendar  │ │  Billing   │ │    SLA     │ │ Contracts  │       │
│  │   Module   │ │   Module   │ │   Module   │ │   Module   │       │
│  └────────────┘ └────────────┘ └────────────┘ └────────────┘       │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐       │
│  │   Assets   │ │ Knowledge  │ │Notifications│ │    RMM     │       │
│  │   (CMDB)   │ │    Base    │ │   Module   │ │Integration │       │
│  └────────────┘ └────────────┘ └────────────┘ └────────────┘       │
└─────────────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────────────┐
│                      CORE SERVICES                                   │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐       │
│  │   Auth     │ │   Audit    │ │   Search   │ │   Files    │       │
│  │  Service   │ │   Logger   │ │   Engine   │ │  Storage   │       │
│  └────────────┘ └────────────┘ └────────────┘ └────────────┘       │
└─────────────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────────────┐
│                      DATA LAYER                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    PostgreSQL 18                             │   │
│  │   (Row-Level Security for Multi-Tenant Isolation)           │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Core Modules Specification

### 3.1 Authentication & Authorization Module

**Purpose**: Secure access control for all platform users

#### 3.1.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Local username/password authentication | P0 | 1 |
| Password policies (complexity, expiry) | P0 | 1 |
| Multi-factor authentication (TOTP) | P0 | 1 |
| Session management | P0 | 1 |
| SSO - SAML 2.0 | P1 | 1 |
| SSO - OIDC | P1 | 1 |
| Active Directory integration | P1 | 1 |
| API key management | P0 | 1 |
| OAuth2 for third-party integrations | P1 | 1 |

#### 3.1.2 Standard Roles

| Role | Description | Permissions |
|------|-------------|-------------|
| Super Admin | Platform-level administrator (SaaS only) | All permissions, tenant management |
| Admin | MSP organization administrator | Full org access, user management |
| Manager | Team/department manager | Team oversight, reports, approvals |
| Technician | Service delivery staff | Tickets, time, projects |
| Dispatcher | Resource scheduling | Calendar, assignments, scheduling |
| Sales | Account management | Contacts, contracts, client data |
| Finance | Billing and invoicing | Billing, payments, financial reports |
| Client Admin | Client organization admin | Client portal admin |
| Client User | End user at client | Limited portal access |

#### 3.1.3 API Endpoints

```
POST   /api/v1/auth/login
POST   /api/v1/auth/logout
POST   /api/v1/auth/refresh
POST   /api/v1/auth/mfa/setup
POST   /api/v1/auth/mfa/verify
GET    /api/v1/auth/sso/providers
POST   /api/v1/auth/sso/{provider}/initiate
POST   /api/v1/auth/sso/{provider}/callback
GET    /api/v1/auth/sessions
DELETE /api/v1/auth/sessions/{id}
```

---

### 3.2 Tenant Management Module (Multi-Tenant Only)

**Purpose**: Manage MSP organizations in SaaS deployment

#### 3.2.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Tenant provisioning | P0 | 1 |
| Tenant configuration | P0 | 1 |
| Tenant suspension/activation | P0 | 1 |
| Usage metering | P1 | 1 |
| Tenant data isolation (RLS) | P0 | 1 |
| Custom domain support | P2 | 1 |
| Tenant backup/export | P1 | 1 |

#### 3.2.2 API Endpoints

```
GET    /api/v1/tenants
POST   /api/v1/tenants
GET    /api/v1/tenants/{id}
PUT    /api/v1/tenants/{id}
DELETE /api/v1/tenants/{id}
POST   /api/v1/tenants/{id}/suspend
POST   /api/v1/tenants/{id}/activate
GET    /api/v1/tenants/{id}/usage
```

---

### 3.3 Contact Management Module

**Purpose**: Manage companies (clients) and their contacts

#### 3.3.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Company CRUD | P0 | 1 |
| Contact CRUD | P0 | 1 |
| Company hierarchy (parent/child) | P1 | 1 |
| Contact roles and types | P0 | 1 |
| Company sites/locations | P0 | 1 |
| Custom fields | P1 | 1 |
| Tags and categories | P1 | 1 |
| Import from CSV | P0 | 1 |
| Import from other PSAs | P1 | 1 |
| Activity history | P0 | 1 |
| Notes and attachments | P0 | 1 |
| Merge duplicate contacts | P1 | 1 |

#### 3.3.2 Data Model

```
Company
├── id: UUID
├── tenant_id: UUID (multi-tenant)
├── name: String
├── parent_company_id: UUID (nullable)
├── company_type: Enum (Client, Prospect, Vendor, Partner)
├── status: Enum (Active, Inactive, Prospect)
├── industry: String
├── website: String
├── phone: String
├── address: Address
├── billing_address: Address
├── tax_id: String
├── default_billing_contact_id: UUID
├── default_technical_contact_id: UUID
├── account_manager_id: UUID
├── contract_id: UUID
├── sla_id: UUID
├── custom_fields: JSONB
├── tags: String[]
├── notes: Text
├── created_at: Timestamp
├── updated_at: Timestamp

Contact
├── id: UUID
├── tenant_id: UUID
├── company_id: UUID
├── first_name: String
├── last_name: String
├── email: String
├── phone: String
├── mobile: String
├── title: String
├── department: String
├── contact_type: Enum (Primary, Technical, Billing, Other)
├── is_portal_user: Boolean
├── portal_user_id: UUID (nullable)
├── preferred_contact_method: Enum
├── timezone: String
├── custom_fields: JSONB
├── tags: String[]
├── notes: Text
├── created_at: Timestamp
├── updated_at: Timestamp

Site
├── id: UUID
├── tenant_id: UUID
├── company_id: UUID
├── name: String
├── address: Address
├── phone: String
├── is_primary: Boolean
├── timezone: String
├── notes: Text
```

#### 3.3.3 API Endpoints

```
# Companies
GET    /api/v1/companies
POST   /api/v1/companies
GET    /api/v1/companies/{id}
PUT    /api/v1/companies/{id}
DELETE /api/v1/companies/{id}
GET    /api/v1/companies/{id}/contacts
GET    /api/v1/companies/{id}/sites
GET    /api/v1/companies/{id}/tickets
GET    /api/v1/companies/{id}/assets
GET    /api/v1/companies/{id}/contracts
GET    /api/v1/companies/{id}/invoices
GET    /api/v1/companies/{id}/activity

# Contacts
GET    /api/v1/contacts
POST   /api/v1/contacts
GET    /api/v1/contacts/{id}
PUT    /api/v1/contacts/{id}
DELETE /api/v1/contacts/{id}
GET    /api/v1/contacts/{id}/tickets
GET    /api/v1/contacts/{id}/activity
POST   /api/v1/contacts/{id}/portal-access

# Sites
GET    /api/v1/sites
POST   /api/v1/sites
GET    /api/v1/sites/{id}
PUT    /api/v1/sites/{id}
DELETE /api/v1/sites/{id}

# Import
POST   /api/v1/contacts/import/csv
POST   /api/v1/contacts/import/{psa-provider}
```

#### 3.3.4 Module Interactions

| Interacts With | Interaction Type | Description |
|---------------|------------------|-------------|
| Ticketing | Provides | Contact/company for ticket assignment |
| Billing | Provides | Billing contact and address |
| Contracts | Provides | Company for contract association |
| Assets | Provides | Company/site for asset location |
| Projects | Provides | Company for project assignment |
| Client Portal | Provides | Portal user associations |

---

### 3.4 Ticketing Module

**Purpose**: Core service desk functionality for issue tracking and resolution

#### 3.4.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Ticket CRUD | P0 | 1 |
| Ticket queues/boards | P0 | 1 |
| Ticket assignment | P0 | 1 |
| Ticket priorities | P0 | 1 |
| Ticket statuses (customizable) | P0 | 1 |
| Ticket types/categories | P0 | 1 |
| Email-to-ticket | P0 | 1 |
| Email parsing rules | P1 | 1 |
| Ticket templates | P1 | 1 |
| Canned responses | P1 | 1 |
| Internal notes | P0 | 1 |
| Public notes (client visible) | P0 | 1 |
| Attachments | P0 | 1 |
| Ticket merging | P1 | 1 |
| Ticket splitting | P2 | 1 |
| Parent/child tickets | P1 | 1 |
| Ticket automation rules | P0 | 1 |
| Auto-assignment rules | P0 | 1 |
| Escalation rules | P0 | 1 |
| SLA tracking | P0 | 1 |
| Time entry from ticket | P0 | 1 |
| Ticket scheduling | P1 | 1 |
| Recurring tickets | P1 | 1 |
| Ticket search/filters | P0 | 1 |
| Custom fields | P1 | 1 |

#### 3.4.2 Data Model

```
Ticket
├── id: UUID
├── tenant_id: UUID
├── ticket_number: String (auto-generated, tenant-unique)
├── title: String
├── description: Text (rich text)
├── status_id: UUID
├── priority_id: UUID
├── type_id: UUID
├── category_id: UUID
├── subcategory_id: UUID
├── source: Enum (Portal, Email, Phone, API, Chat, RMM)
├── company_id: UUID
├── contact_id: UUID
├── site_id: UUID
├── assigned_to_id: UUID (nullable)
├── team_id: UUID (nullable)
├── queue_id: UUID
├── parent_ticket_id: UUID (nullable)
├── contract_id: UUID (nullable)
├── asset_id: UUID (nullable)
├── sla_id: UUID
├── sla_due_date: Timestamp
├── first_response_due: Timestamp
├── first_response_at: Timestamp
├── resolution_due: Timestamp
├── resolved_at: Timestamp
├── closed_at: Timestamp
├── scheduled_start: Timestamp
├── scheduled_end: Timestamp
├── estimated_hours: Decimal
├── actual_hours: Decimal (calculated)
├── is_billable: Boolean
├── billing_status: Enum (NotBilled, ReadyToBill, Billed)
├── custom_fields: JSONB
├── tags: String[]
├── created_by_id: UUID
├── created_at: Timestamp
├── updated_at: Timestamp

TicketStatus
├── id: UUID
├── tenant_id: UUID
├── name: String
├── color: String
├── is_closed: Boolean
├── is_default: Boolean
├── sort_order: Integer

TicketPriority
├── id: UUID
├── tenant_id: UUID
├── name: String
├── color: String
├── sla_multiplier: Decimal
├── sort_order: Integer

TicketNote
├── id: UUID
├── tenant_id: UUID
├── ticket_id: UUID
├── note_type: Enum (Internal, Public, Resolution)
├── content: Text (rich text)
├── is_email_sent: Boolean
├── created_by_id: UUID
├── created_at: Timestamp
├── updated_at: Timestamp

TicketAttachment
├── id: UUID
├── tenant_id: UUID
├── ticket_id: UUID
├── note_id: UUID (nullable)
├── file_name: String
├── file_size: Integer
├── mime_type: String
├── storage_path: String
├── uploaded_by_id: UUID
├── created_at: Timestamp

TicketAutomationRule
├── id: UUID
├── tenant_id: UUID
├── name: String
├── description: Text
├── is_active: Boolean
├── trigger_type: Enum (OnCreate, OnUpdate, OnSchedule, OnSLABreach)
├── conditions: JSONB
├── actions: JSONB
├── priority: Integer
├── created_at: Timestamp
├── updated_at: Timestamp

EmailParseRule
├── id: UUID
├── tenant_id: UUID
├── name: String
├── mailbox_id: UUID
├── conditions: JSONB (from, subject, body patterns)
├── actions: JSONB (set company, contact, priority, assign, etc.)
├── priority: Integer
├── is_active: Boolean
```

#### 3.4.3 Automation Rule Engine

The automation system supports:

**Triggers:**
- On ticket creation
- On ticket update (specific field changes)
- On schedule (cron-based)
- On SLA approaching/breached
- On no response (aging)

**Conditions:**
- Field comparisons (equals, contains, regex)
- Time-based (created within, updated within)
- SLA status
- Ticket age
- Contact/company attributes
- Custom field values

**Actions:**
- Set field values
- Assign to user/team
- Change queue
- Add note
- Send notification
- Create child ticket
- Escalate
- Apply SLA
- Run webhook

#### 3.4.4 API Endpoints

```
# Tickets
GET    /api/v1/tickets
POST   /api/v1/tickets
GET    /api/v1/tickets/{id}
PUT    /api/v1/tickets/{id}
DELETE /api/v1/tickets/{id}
POST   /api/v1/tickets/{id}/notes
GET    /api/v1/tickets/{id}/notes
POST   /api/v1/tickets/{id}/attachments
GET    /api/v1/tickets/{id}/attachments
GET    /api/v1/tickets/{id}/time-entries
GET    /api/v1/tickets/{id}/activity
POST   /api/v1/tickets/{id}/merge
POST   /api/v1/tickets/{id}/split
POST   /api/v1/tickets/{id}/assign
POST   /api/v1/tickets/{id}/escalate

# Queues
GET    /api/v1/ticket-queues
POST   /api/v1/ticket-queues
PUT    /api/v1/ticket-queues/{id}
DELETE /api/v1/ticket-queues/{id}

# Statuses, Priorities, Types
GET    /api/v1/ticket-statuses
POST   /api/v1/ticket-statuses
GET    /api/v1/ticket-priorities
POST   /api/v1/ticket-priorities
GET    /api/v1/ticket-types
POST   /api/v1/ticket-types

# Automation
GET    /api/v1/ticket-automation-rules
POST   /api/v1/ticket-automation-rules
PUT    /api/v1/ticket-automation-rules/{id}
DELETE /api/v1/ticket-automation-rules/{id}

# Email Integration
GET    /api/v1/email-mailboxes
POST   /api/v1/email-mailboxes
PUT    /api/v1/email-mailboxes/{id}
DELETE /api/v1/email-mailboxes/{id}
GET    /api/v1/email-parse-rules
POST   /api/v1/email-parse-rules
```

#### 3.4.5 Module Interactions

| Interacts With | Interaction Type | Description |
|---------------|------------------|-------------|
| Contacts | Consumes | Get company/contact for ticket |
| Time Tracking | Provides | Ticket reference for time entries |
| Billing | Provides | Billable tickets for invoicing |
| SLA | Consumes | Apply SLA policies to tickets |
| Contracts | Consumes | Get applicable contract |
| Assets | Consumes | Link ticket to asset |
| Knowledge Base | Both | Create KB from ticket, suggest KB articles |
| Calendar | Provides | Scheduled ticket appointments |
| Notifications | Triggers | Send alerts on ticket events |
| RMM | Consumes | Auto-create tickets from RMM alerts |

---

### 3.5 Time Tracking Module

**Purpose**: Track billable and non-billable time for services

#### 3.5.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Time entry CRUD | P0 | 1 |
| Timer (start/stop) | P0 | 1 |
| Associate with ticket | P0 | 1 |
| Associate with project/task | P0 | 1 |
| Work types (categories) | P0 | 1 |
| Billable/non-billable | P0 | 1 |
| Time rounding rules | P1 | 1 |
| Time approval workflow | P1 | 1 |
| Minimum time increments | P1 | 1 |
| Time reports | P0 | 1 |
| Timesheet view | P0 | 1 |
| Bulk time entry | P1 | 1 |
| Time entry templates | P2 | 1 |

#### 3.5.2 Data Model

```
TimeEntry
├── id: UUID
├── tenant_id: UUID
├── user_id: UUID
├── date: Date
├── start_time: Time (nullable, for timer)
├── end_time: Time (nullable, for timer)
├── duration_minutes: Integer
├── work_type_id: UUID
├── ticket_id: UUID (nullable)
├── project_id: UUID (nullable)
├── task_id: UUID (nullable)
├── company_id: UUID
├── contract_id: UUID (nullable)
├── notes: Text
├── internal_notes: Text
├── is_billable: Boolean
├── billing_status: Enum (NotBilled, ReadyToBill, Billed)
├── invoice_id: UUID (nullable)
├── hourly_rate: Decimal
├── total_amount: Decimal
├── approval_status: Enum (Pending, Approved, Rejected)
├── approved_by_id: UUID (nullable)
├── approved_at: Timestamp
├── created_at: Timestamp
├── updated_at: Timestamp

WorkType
├── id: UUID
├── tenant_id: UUID
├── name: String
├── description: Text
├── default_billable: Boolean
├── default_rate: Decimal
├── is_active: Boolean

TimeRoundingRule
├── id: UUID
├── tenant_id: UUID
├── name: String
├── increment_minutes: Integer (e.g., 15)
├── rounding_method: Enum (Up, Down, Nearest)
├── minimum_minutes: Integer
├── is_default: Boolean
```

#### 3.5.3 API Endpoints

```
# Time Entries
GET    /api/v1/time-entries
POST   /api/v1/time-entries
GET    /api/v1/time-entries/{id}
PUT    /api/v1/time-entries/{id}
DELETE /api/v1/time-entries/{id}

# Timer
POST   /api/v1/time-entries/timer/start
POST   /api/v1/time-entries/timer/stop
GET    /api/v1/time-entries/timer/active

# Timesheet
GET    /api/v1/timesheets/{user_id}/{start_date}/{end_date}
POST   /api/v1/timesheets/submit
POST   /api/v1/timesheets/approve
POST   /api/v1/timesheets/reject

# Work Types
GET    /api/v1/work-types
POST   /api/v1/work-types
PUT    /api/v1/work-types/{id}
DELETE /api/v1/work-types/{id}

# Reports
GET    /api/v1/time-entries/reports/by-user
GET    /api/v1/time-entries/reports/by-company
GET    /api/v1/time-entries/reports/by-project
GET    /api/v1/time-entries/reports/billable-summary
```

---

### 3.6 Project & Task Management Module

**Purpose**: Manage client projects, internal projects, and tasks

#### 3.6.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Project CRUD | P0 | 1 |
| Project templates | P1 | 1 |
| Project phases/milestones | P1 | 1 |
| Task CRUD | P0 | 1 |
| Task dependencies | P1 | 1 |
| Task assignment | P0 | 1 |
| Task checklists | P1 | 1 |
| Kanban board view | P0 | 1 |
| Gantt chart view | P2 | 1 |
| Project budgeting | P1 | 1 |
| Project time tracking | P0 | 1 |
| Project billing | P1 | 1 |
| Resource allocation | P1 | 1 |
| Project status reports | P1 | 1 |

#### 3.6.2 Data Model

```
Project
├── id: UUID
├── tenant_id: UUID
├── name: String
├── description: Text
├── project_number: String
├── company_id: UUID
├── contract_id: UUID (nullable)
├── project_type: Enum (Client, Internal)
├── status: Enum (Planning, Active, OnHold, Completed, Cancelled)
├── project_manager_id: UUID
├── start_date: Date
├── target_end_date: Date
├── actual_end_date: Date (nullable)
├── budget_hours: Decimal
├── budget_amount: Decimal
├── billing_method: Enum (FixedPrice, TimeAndMaterials, NotBillable)
├── hourly_rate: Decimal
├── is_billable: Boolean
├── custom_fields: JSONB
├── created_at: Timestamp
├── updated_at: Timestamp

ProjectPhase
├── id: UUID
├── tenant_id: UUID
├── project_id: UUID
├── name: String
├── description: Text
├── sort_order: Integer
├── start_date: Date
├── end_date: Date
├── status: Enum (NotStarted, InProgress, Completed)

Task
├── id: UUID
├── tenant_id: UUID
├── project_id: UUID (nullable)
├── phase_id: UUID (nullable)
├── parent_task_id: UUID (nullable)
├── title: String
├── description: Text
├── status_id: UUID
├── priority: Enum (Low, Medium, High, Critical)
├── assigned_to_id: UUID (nullable)
├── estimated_hours: Decimal
├── actual_hours: Decimal (calculated)
├── start_date: Date
├── due_date: Date
├── completed_at: Timestamp
├── sort_order: Integer
├── checklist: JSONB
├── custom_fields: JSONB
├── created_at: Timestamp
├── updated_at: Timestamp

TaskDependency
├── id: UUID
├── tenant_id: UUID
├── task_id: UUID
├── depends_on_task_id: UUID
├── dependency_type: Enum (FinishToStart, StartToStart, FinishToFinish)

TaskStatus
├── id: UUID
├── tenant_id: UUID
├── name: String
├── color: String
├── is_completed: Boolean
├── sort_order: Integer
```

#### 3.6.3 API Endpoints

```
# Projects
GET    /api/v1/projects
POST   /api/v1/projects
GET    /api/v1/projects/{id}
PUT    /api/v1/projects/{id}
DELETE /api/v1/projects/{id}
GET    /api/v1/projects/{id}/tasks
GET    /api/v1/projects/{id}/phases
GET    /api/v1/projects/{id}/time-entries
GET    /api/v1/projects/{id}/budget-status

# Tasks
GET    /api/v1/tasks
POST   /api/v1/tasks
GET    /api/v1/tasks/{id}
PUT    /api/v1/tasks/{id}
DELETE /api/v1/tasks/{id}
POST   /api/v1/tasks/{id}/assign
POST   /api/v1/tasks/{id}/complete
GET    /api/v1/tasks/kanban/{board_id}

# Project Templates
GET    /api/v1/project-templates
POST   /api/v1/project-templates
POST   /api/v1/projects/from-template/{template_id}
```

---

### 3.7 Calendar & Scheduling Module

**Purpose**: Schedule appointments, dispatch resources, and manage availability

#### 3.7.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Calendar views (day, week, month) | P0 | 1 |
| Appointment CRUD | P0 | 1 |
| Recurring appointments | P1 | 1 |
| Resource scheduling | P0 | 1 |
| Dispatch board | P0 | 1 |
| Availability management | P1 | 1 |
| Schedule conflicts detection | P0 | 1 |
| Travel time calculation | P2 | 1 |
| Drag-and-drop scheduling | P1 | 1 |
| Calendar sync (Google, Outlook) | P2 | 1 |
| On-call schedules | P1 | 1 |

#### 3.7.2 Data Model

```
Appointment
├── id: UUID
├── tenant_id: UUID
├── title: String
├── description: Text
├── appointment_type: Enum (Ticket, Project, Meeting, Other)
├── ticket_id: UUID (nullable)
├── project_id: UUID (nullable)
├── task_id: UUID (nullable)
├── company_id: UUID (nullable)
├── contact_id: UUID (nullable)
├── site_id: UUID (nullable)
├── assigned_to_id: UUID
├── start_time: Timestamp
├── end_time: Timestamp
├── all_day: Boolean
├── status: Enum (Scheduled, InProgress, Completed, Cancelled)
├── location: String
├── notes: Text
├── recurrence_rule: String (iCal RRULE)
├── recurrence_parent_id: UUID (nullable)
├── reminder_minutes: Integer[]
├── created_at: Timestamp
├── updated_at: Timestamp

UserAvailability
├── id: UUID
├── tenant_id: UUID
├── user_id: UUID
├── day_of_week: Integer (0-6)
├── start_time: Time
├── end_time: Time
├── is_available: Boolean

TimeOff
├── id: UUID
├── tenant_id: UUID
├── user_id: UUID
├── start_date: Date
├── end_date: Date
├── type: Enum (Vacation, Sick, Personal, Holiday)
├── status: Enum (Pending, Approved, Rejected)
├── approved_by_id: UUID (nullable)
├── notes: Text

OnCallSchedule
├── id: UUID
├── tenant_id: UUID
├── name: String
├── team_id: UUID
├── rotation_type: Enum (Weekly, Daily, Custom)
├── rotation_config: JSONB
├── is_active: Boolean
```

#### 3.7.3 API Endpoints

```
# Appointments
GET    /api/v1/appointments
POST   /api/v1/appointments
GET    /api/v1/appointments/{id}
PUT    /api/v1/appointments/{id}
DELETE /api/v1/appointments/{id}
GET    /api/v1/appointments/calendar/{start_date}/{end_date}
GET    /api/v1/appointments/user/{user_id}/{start_date}/{end_date}

# Dispatch
GET    /api/v1/dispatch/board/{date}
GET    /api/v1/dispatch/resources/available
POST   /api/v1/dispatch/assign

# Availability
GET    /api/v1/availability/{user_id}
PUT    /api/v1/availability/{user_id}
GET    /api/v1/time-off
POST   /api/v1/time-off
PUT    /api/v1/time-off/{id}

# On-Call
GET    /api/v1/on-call-schedules
POST   /api/v1/on-call-schedules
GET    /api/v1/on-call/current
```

---

### 3.8 Contracts & Agreements Module

**Purpose**: Manage service agreements, contracts, and their terms

#### 3.8.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Contract CRUD | P0 | 1 |
| Contract types (MSA, SOW, etc.) | P0 | 1 |
| Contract terms and conditions | P1 | 1 |
| Contract items/services | P0 | 1 |
| Prepaid/block hours tracking | P0 | 1 |
| Contract renewals | P1 | 1 |
| Contract expiration alerts | P0 | 1 |
| Rate cards | P0 | 1 |
| Contract documents | P1 | 1 |
| Contract versioning | P2 | 1 |

#### 3.8.2 Data Model

```
Contract
├── id: UUID
├── tenant_id: UUID
├── contract_number: String
├── name: String
├── company_id: UUID
├── contract_type: Enum (ManagedServices, BlockHours, TimeAndMaterials, FixedPrice, Warranty)
├── status: Enum (Draft, Active, Expired, Cancelled, Renewed)
├── start_date: Date
├── end_date: Date
├── auto_renew: Boolean
├── renewal_terms: JSONB
├── billing_cycle: Enum (Monthly, Quarterly, Annually)
├── billing_amount: Decimal
├── sla_id: UUID (nullable)
├── signed_date: Date
├── signed_by_contact_id: UUID (nullable)
├── notes: Text
├── custom_fields: JSONB
├── created_at: Timestamp
├── updated_at: Timestamp

ContractItem
├── id: UUID
├── tenant_id: UUID
├── contract_id: UUID
├── name: String
├── description: Text
├── item_type: Enum (RecurringService, BlockHours, Retainer, Product)
├── quantity: Decimal
├── unit_price: Decimal
├── total_price: Decimal
├── billing_frequency: Enum
├── work_type_id: UUID (nullable)
├── included_hours: Decimal (for block hour contracts)
├── overage_rate: Decimal
├── sort_order: Integer

ContractHourBalance
├── id: UUID
├── tenant_id: UUID
├── contract_id: UUID
├── contract_item_id: UUID
├── period_start: Date
├── period_end: Date
├── hours_included: Decimal
├── hours_used: Decimal
├── hours_remaining: Decimal
├── rollover_hours: Decimal
├── created_at: Timestamp
├── updated_at: Timestamp

RateCard
├── id: UUID
├── tenant_id: UUID
├── name: String
├── description: Text
├── is_default: Boolean

RateCardItem
├── id: UUID
├── rate_card_id: UUID
├── work_type_id: UUID
├── hourly_rate: Decimal
├── after_hours_rate: Decimal
├── emergency_rate: Decimal
```

#### 3.8.3 API Endpoints

```
# Contracts
GET    /api/v1/contracts
POST   /api/v1/contracts
GET    /api/v1/contracts/{id}
PUT    /api/v1/contracts/{id}
DELETE /api/v1/contracts/{id}
GET    /api/v1/contracts/{id}/items
POST   /api/v1/contracts/{id}/items
GET    /api/v1/contracts/{id}/hour-balance
POST   /api/v1/contracts/{id}/renew

# Rate Cards
GET    /api/v1/rate-cards
POST   /api/v1/rate-cards
GET    /api/v1/rate-cards/{id}
PUT    /api/v1/rate-cards/{id}
DELETE /api/v1/rate-cards/{id}
```

---

### 3.9 SLA Management Module

**Purpose**: Define and track Service Level Agreements

#### 3.9.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| SLA policy CRUD | P0 | 1 |
| Response time targets | P0 | 1 |
| Resolution time targets | P0 | 1 |
| Business hours support | P0 | 1 |
| Priority-based SLA targets | P0 | 1 |
| SLA breach notifications | P0 | 1 |
| SLA approaching warnings | P0 | 1 |
| SLA reports | P1 | 1 |
| SLA exclusions (holidays, etc.) | P1 | 1 |

#### 3.9.2 Data Model

```
SLAPolicy
├── id: UUID
├── tenant_id: UUID
├── name: String
├── description: Text
├── is_default: Boolean
├── business_hours_id: UUID
├── created_at: Timestamp
├── updated_at: Timestamp

SLATarget
├── id: UUID
├── sla_policy_id: UUID
├── priority_id: UUID
├── first_response_hours: Decimal
├── resolution_hours: Decimal
├── operational_hours: Enum (BusinessHours, 24x7)

BusinessHours
├── id: UUID
├── tenant_id: UUID
├── name: String
├── timezone: String
├── schedule: JSONB (day -> start/end times)
├── holidays: UUID[] (reference to holiday calendar)

HolidayCalendar
├── id: UUID
├── tenant_id: UUID
├── name: String
├── holidays: JSONB (array of dates and names)
```

#### 3.9.3 API Endpoints

```
GET    /api/v1/sla-policies
POST   /api/v1/sla-policies
GET    /api/v1/sla-policies/{id}
PUT    /api/v1/sla-policies/{id}
DELETE /api/v1/sla-policies/{id}
GET    /api/v1/sla-policies/{id}/targets

GET    /api/v1/business-hours
POST   /api/v1/business-hours
PUT    /api/v1/business-hours/{id}

GET    /api/v1/holiday-calendars
POST   /api/v1/holiday-calendars
PUT    /api/v1/holiday-calendars/{id}

# SLA Reports
GET    /api/v1/sla/reports/compliance
GET    /api/v1/sla/reports/breaches
```

---

### 3.10 Billing & Invoicing Module

**Purpose**: Generate invoices and manage billing

#### 3.10.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Invoice CRUD | P0 | 1 |
| Invoice generation from time entries | P0 | 1 |
| Invoice generation from contracts | P0 | 1 |
| Invoice templates | P1 | 1 |
| Batch invoicing | P1 | 1 |
| Invoice approval workflow | P1 | 1 |
| Credit memos | P1 | 1 |
| Payment recording | P0 | 1 |
| Payment gateway integration (Stripe) | P0 | 1 |
| ACH payment support | P1 | 1 |
| Invoice email delivery | P0 | 1 |
| Invoice PDF generation | P0 | 1 |
| Accounts receivable aging | P0 | 1 |
| Late payment reminders | P1 | 1 |
| Tax calculation | P1 | 1 |
| Revenue recognition | P2 | 1 |

#### 3.10.2 Data Model

```
Invoice
├── id: UUID
├── tenant_id: UUID
├── invoice_number: String
├── company_id: UUID
├── billing_contact_id: UUID
├── contract_id: UUID (nullable)
├── status: Enum (Draft, Pending, Sent, Paid, PartiallyPaid, Void, WrittenOff)
├── invoice_date: Date
├── due_date: Date
├── payment_terms: Enum (DueOnReceipt, Net15, Net30, Net45, Net60)
├── subtotal: Decimal
├── tax_amount: Decimal
├── discount_amount: Decimal
├── total: Decimal
├── amount_paid: Decimal
├── balance_due: Decimal
├── currency: String (USD)
├── notes: Text
├── internal_notes: Text
├── po_number: String
├── sent_at: Timestamp
├── paid_at: Timestamp
├── created_at: Timestamp
├── updated_at: Timestamp

InvoiceLine
├── id: UUID
├── invoice_id: UUID
├── line_type: Enum (Service, Product, TimeEntry, Adjustment, Tax, Discount)
├── description: String
├── quantity: Decimal
├── unit_price: Decimal
├── total: Decimal
├── time_entry_ids: UUID[] (for time-based lines)
├── ticket_id: UUID (nullable)
├── project_id: UUID (nullable)
├── sort_order: Integer

Payment
├── id: UUID
├── tenant_id: UUID
├── invoice_id: UUID (nullable, for unapplied payments)
├── company_id: UUID
├── payment_date: Date
├── amount: Decimal
├── payment_method: Enum (Check, CreditCard, ACH, Wire, Cash, Other)
├── reference_number: String
├── gateway_transaction_id: String (nullable)
├── notes: Text
├── created_at: Timestamp

PaymentGatewayConfig
├── id: UUID
├── tenant_id: UUID
├── provider: Enum (Stripe, AuthorizeNet, PayPal)
├── is_active: Boolean
├── config: JSONB (encrypted API keys, etc.)
├── is_test_mode: Boolean

TaxRate
├── id: UUID
├── tenant_id: UUID
├── name: String
├── rate: Decimal
├── is_default: Boolean
├── is_active: Boolean
```

#### 3.10.3 API Endpoints

```
# Invoices
GET    /api/v1/invoices
POST   /api/v1/invoices
GET    /api/v1/invoices/{id}
PUT    /api/v1/invoices/{id}
DELETE /api/v1/invoices/{id}
POST   /api/v1/invoices/{id}/send
POST   /api/v1/invoices/{id}/void
GET    /api/v1/invoices/{id}/pdf

# Invoice Generation
POST   /api/v1/invoices/generate/from-time-entries
POST   /api/v1/invoices/generate/from-contract
POST   /api/v1/invoices/generate/batch

# Payments
GET    /api/v1/payments
POST   /api/v1/payments
GET    /api/v1/payments/{id}
POST   /api/v1/payments/process-card

# Payment Gateway
GET    /api/v1/payment-gateway/config
PUT    /api/v1/payment-gateway/config
POST   /api/v1/payment-gateway/test

# Reports
GET    /api/v1/billing/reports/ar-aging
GET    /api/v1/billing/reports/revenue
GET    /api/v1/billing/reports/collections
```

---

### 3.11 Asset Management (CMDB) Module

**Purpose**: Track client assets, configurations, and relationships

#### 3.11.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Asset CRUD | P0 | 1 |
| Asset types (customizable) | P0 | 1 |
| Asset-contact association | P0 | 1 |
| Asset-site association | P0 | 1 |
| Asset relationships | P1 | 1 |
| Asset lifecycle tracking | P1 | 1 |
| Warranty tracking | P0 | 1 |
| Configuration items | P1 | 1 |
| Asset sync from RMM | P0 | 1 |
| Asset audit history | P0 | 1 |
| Asset documentation | P1 | 1 |
| Password/credential vault | P1 | 1 |
| Network documentation | P1 | 1 |

#### 3.11.2 Data Model

```
AssetType
├── id: UUID
├── tenant_id: UUID
├── name: String
├── icon: String
├── parent_type_id: UUID (nullable)
├── custom_fields_schema: JSONB
├── is_active: Boolean

Asset
├── id: UUID
├── tenant_id: UUID
├── asset_tag: String
├── name: String
├── asset_type_id: UUID
├── company_id: UUID
├── site_id: UUID (nullable)
├── contact_id: UUID (nullable, primary user)
├── status: Enum (Active, Inactive, Retired, InRepair, InStock)
├── manufacturer: String
├── model: String
├── serial_number: String
├── purchase_date: Date
├── purchase_price: Decimal
├── warranty_expiry: Date
├── end_of_life: Date
├── rmm_device_id: String (nullable)
├── last_sync_at: Timestamp (nullable)
├── custom_fields: JSONB
├── notes: Text
├── created_at: Timestamp
├── updated_at: Timestamp

AssetRelationship
├── id: UUID
├── tenant_id: UUID
├── parent_asset_id: UUID
├── child_asset_id: UUID
├── relationship_type: Enum (Contains, ConnectedTo, DependsOn, Hosts)

ConfigurationItem
├── id: UUID
├── tenant_id: UUID
├── asset_id: UUID
├── name: String
├── category: String
├── value: Text (encrypted)
├── notes: Text
├── created_at: Timestamp
├── updated_at: Timestamp

CredentialVault
├── id: UUID
├── tenant_id: UUID
├── name: String
├── company_id: UUID (nullable)
├── asset_id: UUID (nullable)
├── credential_type: Enum (LocalAdmin, Domain, SSH, API, Other)
├── username: String (encrypted)
├── password: String (encrypted)
├── url: String
├── notes: Text (encrypted)
├── last_rotated: Timestamp
├── created_at: Timestamp
├── updated_at: Timestamp

AssetAuditLog
├── id: UUID
├── tenant_id: UUID
├── asset_id: UUID
├── action: Enum (Created, Updated, Synced, StatusChanged)
├── changes: JSONB
├── performed_by_id: UUID
├── performed_at: Timestamp
```

#### 3.11.3 API Endpoints

```
# Assets
GET    /api/v1/assets
POST   /api/v1/assets
GET    /api/v1/assets/{id}
PUT    /api/v1/assets/{id}
DELETE /api/v1/assets/{id}
GET    /api/v1/assets/{id}/tickets
GET    /api/v1/assets/{id}/relationships
GET    /api/v1/assets/{id}/audit-log
POST   /api/v1/assets/{id}/sync-rmm

# Asset Types
GET    /api/v1/asset-types
POST   /api/v1/asset-types
PUT    /api/v1/asset-types/{id}
DELETE /api/v1/asset-types/{id}

# Credential Vault
GET    /api/v1/credentials
POST   /api/v1/credentials
GET    /api/v1/credentials/{id}
PUT    /api/v1/credentials/{id}
DELETE /api/v1/credentials/{id}
POST   /api/v1/credentials/{id}/reveal
```

---

### 3.12 Knowledge Base Module

**Purpose**: Create and manage documentation and solutions

#### 3.12.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Article CRUD | P0 | 1 |
| Article categories | P0 | 1 |
| Article tagging | P0 | 1 |
| Rich text editor | P0 | 1 |
| Article versioning | P1 | 1 |
| Article visibility (internal/public) | P0 | 1 |
| Full-text search | P0 | 1 |
| Article ratings | P1 | 1 |
| Related articles | P1 | 1 |
| Article templates | P2 | 1 |
| Link articles to tickets | P0 | 1 |
| Generate article from ticket | P1 | 1 |

#### 3.12.2 Data Model

```
KBCategory
├── id: UUID
├── tenant_id: UUID
├── name: String
├── description: Text
├── parent_id: UUID (nullable)
├── slug: String
├── visibility: Enum (Public, Internal, ClientSpecific)
├── sort_order: Integer

KBArticle
├── id: UUID
├── tenant_id: UUID
├── title: String
├── slug: String
├── content: Text (rich text / markdown)
├── summary: Text
├── category_id: UUID
├── visibility: Enum (Public, Internal, ClientSpecific)
├── company_ids: UUID[] (for client-specific)
├── status: Enum (Draft, Published, Archived)
├── author_id: UUID
├── tags: String[]
├── view_count: Integer
├── helpful_count: Integer
├── not_helpful_count: Integer
├── related_article_ids: UUID[]
├── related_ticket_ids: UUID[]
├── published_at: Timestamp
├── created_at: Timestamp
├── updated_at: Timestamp

KBArticleVersion
├── id: UUID
├── article_id: UUID
├── version_number: Integer
├── title: String
├── content: Text
├── edited_by_id: UUID
├── created_at: Timestamp
```

#### 3.12.3 API Endpoints

```
# Articles
GET    /api/v1/kb/articles
POST   /api/v1/kb/articles
GET    /api/v1/kb/articles/{id}
PUT    /api/v1/kb/articles/{id}
DELETE /api/v1/kb/articles/{id}
POST   /api/v1/kb/articles/{id}/publish
POST   /api/v1/kb/articles/{id}/archive
GET    /api/v1/kb/articles/{id}/versions
POST   /api/v1/kb/articles/{id}/helpful
POST   /api/v1/kb/articles/{id}/not-helpful
POST   /api/v1/kb/articles/from-ticket/{ticket_id}

# Categories
GET    /api/v1/kb/categories
POST   /api/v1/kb/categories
PUT    /api/v1/kb/categories/{id}
DELETE /api/v1/kb/categories/{id}

# Search
GET    /api/v1/kb/search
```

---

### 3.13 Notifications Module

**Purpose**: Send notifications across multiple channels

#### 3.13.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Email notifications | P0 | 1 |
| SMS notifications | P1 | 1 |
| Slack integration | P1 | 1 |
| Microsoft Teams integration | P1 | 1 |
| Discord integration | P1 | 1 |
| Google Chat integration | P1 | 1 |
| Mattermost integration | P1 | 1 |
| In-app notifications | P0 | 1 |
| Notification templates | P0 | 1 |
| Notification preferences (per user) | P0 | 1 |
| Notification rules | P1 | 1 |
| Delivery tracking | P1 | 1 |
| Scheduled notifications | P2 | 1 |

#### 3.13.2 Data Model

```
NotificationChannel
├── id: UUID
├── tenant_id: UUID
├── channel_type: Enum (Email, SMS, Slack, Teams, Discord, GoogleChat, Mattermost, InApp)
├── name: String
├── config: JSONB (encrypted)
├── is_active: Boolean
├── is_default: Boolean

NotificationTemplate
├── id: UUID
├── tenant_id: UUID
├── name: String
├── event_type: String (e.g., "ticket.created", "invoice.sent")
├── subject: String (with placeholders)
├── body_text: Text
├── body_html: Text
├── channel_type: Enum
├── is_active: Boolean

UserNotificationPreference
├── id: UUID
├── tenant_id: UUID
├── user_id: UUID
├── event_type: String
├── channel_types: Enum[] (which channels to receive on)
├── is_enabled: Boolean

Notification
├── id: UUID
├── tenant_id: UUID
├── user_id: UUID (nullable, for in-app)
├── channel_type: Enum
├── template_id: UUID (nullable)
├── recipient: String (email, phone, channel ID)
├── subject: String
├── body: Text
├── status: Enum (Pending, Sent, Delivered, Failed)
├── error_message: Text (nullable)
├── sent_at: Timestamp
├── delivered_at: Timestamp
├── read_at: Timestamp (for in-app)
├── created_at: Timestamp

NotificationRule
├── id: UUID
├── tenant_id: UUID
├── name: String
├── event_type: String
├── conditions: JSONB
├── channels: Enum[]
├── recipients: JSONB (user IDs, roles, or dynamic)
├── template_id: UUID
├── is_active: Boolean
```

#### 3.13.3 API Endpoints

```
# Channels
GET    /api/v1/notification-channels
POST   /api/v1/notification-channels
PUT    /api/v1/notification-channels/{id}
DELETE /api/v1/notification-channels/{id}
POST   /api/v1/notification-channels/{id}/test

# Templates
GET    /api/v1/notification-templates
POST   /api/v1/notification-templates
PUT    /api/v1/notification-templates/{id}
DELETE /api/v1/notification-templates/{id}

# User Preferences
GET    /api/v1/users/{id}/notification-preferences
PUT    /api/v1/users/{id}/notification-preferences

# In-App Notifications
GET    /api/v1/notifications
PUT    /api/v1/notifications/{id}/read
PUT    /api/v1/notifications/read-all
GET    /api/v1/notifications/unread-count

# Rules
GET    /api/v1/notification-rules
POST   /api/v1/notification-rules
PUT    /api/v1/notification-rules/{id}
DELETE /api/v1/notification-rules/{id}
```

---

### 3.14 RMM Integration Module (Tactical RMM)

**Purpose**: Integrate with Remote Monitoring and Management tools

#### 3.14.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Tactical RMM API connection | P0 | 1 |
| Agent/device sync to assets | P0 | 1 |
| Alert-to-ticket creation | P0 | 1 |
| Remote control launch | P1 | 1 |
| Script execution | P2 | 1 |
| Device health dashboard | P1 | 1 |
| Automated asset discovery | P1 | 1 |
| Alert suppression rules | P1 | 1 |
| Patch status sync | P2 | 1 |

#### 3.14.2 Data Model

```
RMMConnection
├── id: UUID
├── tenant_id: UUID
├── name: String
├── provider: Enum (TacticalRMM, Datto, ConnectWise, NinjaRMM)
├── api_url: String
├── api_key: String (encrypted)
├── api_secret: String (encrypted)
├── is_active: Boolean
├── sync_interval_minutes: Integer
├── last_sync_at: Timestamp
├── sync_status: Enum (Success, Failed, InProgress)
├── created_at: Timestamp
├── updated_at: Timestamp

RMMDeviceMapping
├── id: UUID
├── tenant_id: UUID
├── rmm_connection_id: UUID
├── rmm_device_id: String
├── asset_id: UUID
├── company_id: UUID
├── last_seen: Timestamp
├── sync_status: Enum

RMMAlertRule
├── id: UUID
├── tenant_id: UUID
├── rmm_connection_id: UUID
├── name: String
├── alert_type: String
├── severity_mapping: JSONB (RMM severity -> ticket priority)
├── auto_create_ticket: Boolean
├── ticket_template_id: UUID (nullable)
├── assign_to_id: UUID (nullable)
├── queue_id: UUID
├── is_active: Boolean
├── suppression_rules: JSONB
```

#### 3.14.3 API Endpoints

```
# Connections
GET    /api/v1/rmm/connections
POST   /api/v1/rmm/connections
PUT    /api/v1/rmm/connections/{id}
DELETE /api/v1/rmm/connections/{id}
POST   /api/v1/rmm/connections/{id}/test
POST   /api/v1/rmm/connections/{id}/sync

# Device Mapping
GET    /api/v1/rmm/devices
GET    /api/v1/rmm/devices/unmapped
POST   /api/v1/rmm/devices/{id}/map
DELETE /api/v1/rmm/devices/{id}/unmap

# Alerts
GET    /api/v1/rmm/alerts
GET    /api/v1/rmm/alert-rules
POST   /api/v1/rmm/alert-rules
PUT    /api/v1/rmm/alert-rules/{id}

# Remote Control
POST   /api/v1/rmm/devices/{id}/remote-control
GET    /api/v1/rmm/devices/{id}/health
```

---

### 3.15 Client Portal Module

**Purpose**: Self-service portal for end clients

#### 3.15.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Portal login (separate from admin) | P0 | 1 |
| View tickets | P0 | 1 |
| Create tickets | P0 | 1 |
| Add notes to tickets | P0 | 1 |
| View invoices | P0 | 1 |
| Pay invoices online | P0 | 1 |
| View assets | P1 | 1 |
| View contracts | P1 | 1 |
| Approve quotes | P2 | 2 |
| Request services | P1 | 1 |
| View knowledge base | P0 | 1 |
| Portal branding (per-tenant) | P1 | 1 |
| Portal announcements | P2 | 1 |
| Contact management | P1 | 1 |

#### 3.15.2 API Endpoints (Client Portal Specific)

```
# Authentication
POST   /api/v1/portal/auth/login
POST   /api/v1/portal/auth/logout
POST   /api/v1/portal/auth/forgot-password
POST   /api/v1/portal/auth/reset-password

# Tickets
GET    /api/v1/portal/tickets
POST   /api/v1/portal/tickets
GET    /api/v1/portal/tickets/{id}
POST   /api/v1/portal/tickets/{id}/notes
GET    /api/v1/portal/tickets/{id}/attachments
POST   /api/v1/portal/tickets/{id}/attachments

# Invoices
GET    /api/v1/portal/invoices
GET    /api/v1/portal/invoices/{id}
GET    /api/v1/portal/invoices/{id}/pdf
POST   /api/v1/portal/invoices/{id}/pay

# Assets
GET    /api/v1/portal/assets

# Knowledge Base
GET    /api/v1/portal/kb/articles
GET    /api/v1/portal/kb/articles/{id}
GET    /api/v1/portal/kb/search

# Company Profile
GET    /api/v1/portal/company
PUT    /api/v1/portal/company/contacts
```

---

### 3.16 Reporting Module (Basic - Phase 1)

**Purpose**: Standard reports for operations and management

#### 3.16.1 Phase 1 Reports

| Report | Description |
|--------|-------------|
| Ticket Summary | Tickets by status, priority, company |
| SLA Compliance | SLA performance metrics |
| Time Utilization | Billable vs non-billable hours |
| Revenue Summary | Invoiced amounts by period |
| AR Aging | Outstanding invoices by age |
| Technician Productivity | Time and tickets per technician |
| Contract Profitability | Revenue vs hours by contract |

#### 3.16.2 API Endpoints

```
GET    /api/v1/reports/tickets/summary
GET    /api/v1/reports/sla/compliance
GET    /api/v1/reports/time/utilization
GET    /api/v1/reports/billing/revenue
GET    /api/v1/reports/billing/ar-aging
GET    /api/v1/reports/productivity/by-technician
GET    /api/v1/reports/contracts/profitability

# Report Export
GET    /api/v1/reports/{report_type}/export?format=csv
GET    /api/v1/reports/{report_type}/export?format=pdf
```

---

### 3.17 Settings & Configuration Module

**Purpose**: System-wide and tenant-specific configuration

#### 3.17.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Tenant settings | P0 | 1 |
| Module enable/disable | P0 | 1 |
| Company branding | P1 | 1 |
| Email settings (SMTP) | P0 | 1 |
| Default values | P0 | 1 |
| Custom fields management | P1 | 1 |
| Workflow configuration | P1 | 1 |
| Audit log viewing | P0 | 1 |
| Data import/export | P1 | 1 |
| API documentation | P0 | 1 |

---

### 3.18 Audit & Security Module

**Purpose**: Track all system activities and ensure security

#### 3.18.1 Features

| Feature | Priority | Phase |
|---------|----------|-------|
| Audit logging (all changes) | P0 | 1 |
| Login/logout logging | P0 | 1 |
| API access logging | P0 | 1 |
| Data access logging | P1 | 1 |
| Audit log search | P0 | 1 |
| Audit log export | P1 | 1 |
| Security alerts | P1 | 1 |
| IP allowlisting | P2 | 1 |
| Session management | P0 | 1 |

#### 3.18.2 Data Model

```
AuditLog
├── id: UUID
├── tenant_id: UUID
├── user_id: UUID
├── action: Enum (Create, Update, Delete, View, Login, Logout, Export)
├── entity_type: String
├── entity_id: UUID
├── old_values: JSONB (nullable)
├── new_values: JSONB (nullable)
├── ip_address: String
├── user_agent: String
├── timestamp: Timestamp
```

---

## 4. Module Interaction Matrix

| Module | Contacts | Tickets | Time | Projects | Calendar | Contracts | SLA | Billing | Assets | KB | Notify | RMM |
|--------|----------|---------|------|----------|----------|-----------|-----|---------|--------|-----|--------|-----|
| Contacts | - | P | P | P | P | P | - | P | P | - | - | - |
| Tickets | C | - | P | C | P | C | C | P | C | B | T | C |
| Time | C | C | - | C | - | C | - | P | - | - | - | - |
| Projects | C | C | C | - | P | C | - | P | - | - | T | - |
| Calendar | C | C | - | C | - | - | - | - | - | - | T | - |
| Contracts | C | P | P | P | - | - | P | P | - | - | T | - |
| SLA | - | C | - | - | - | C | - | - | - | - | T | - |
| Billing | C | C | C | C | - | C | - | - | - | - | T | - |
| Assets | C | P | - | - | - | - | - | - | - | - | - | C |
| KB | - | B | - | - | - | - | - | - | - | - | - | - |
| Notify | - | C | - | C | C | C | C | C | - | - | - | C |
| RMM | P | P | - | - | - | - | - | - | P | - | T | - |

**Legend:** P = Provides data, C = Consumes data, B = Bidirectional, T = Triggers events

---

## 5. Phase 2 Features (Future)

### 5.1 Advanced Reporting & BI Dashboards
- Custom report builder
- Drag-and-drop dashboard designer
- Scheduled report delivery
- Data visualization library
- KPI tracking and goals

### 5.2 Quoting & Proposals
- Quote CRUD with line items
- Quote templates
- E-signature integration
- Quote approval workflow
- Quote-to-project conversion
- Quote-to-contract conversion

### 5.3 Procurement & Purchasing
- Product catalog
- Vendor management
- Purchase orders
- Receiving
- Inventory tracking
- Markup/margin calculation

### 5.4 Mobile Application
- Native iOS and Android apps
- Push notifications
- Offline capability
- Time tracking
- Ticket management
- Barcode/QR scanning for assets

### 5.5 Additional RMM Integrations
- Datto RMM
- ConnectWise Automate
- NinjaRMM
- Syncro

### 5.6 PSA Data Import
- ConnectWise Manage import
- Autotask import
- HaloPSA import
- Custom CSV mapping

---

## 6. Non-Functional Requirements

### 6.1 Performance

| Metric | Target |
|--------|--------|
| API response time (p95) | < 200ms |
| Page load time | < 2 seconds |
| Concurrent users per tenant | 100 |
| Total concurrent users (SaaS) | 10,000 |

### 6.2 Scalability

- Horizontal scaling for API servers
- Database read replicas
- Connection pooling
- Caching layer (Redis)
- CDN for static assets

### 6.3 Availability

| Target | Value |
|--------|-------|
| Uptime SLA | 99.9% |
| Recovery Time Objective | 1 hour |
| Recovery Point Objective | 5 minutes |

### 6.4 Security

- HTTPS everywhere
- Data encryption at rest (AES-256)
- Data encryption in transit (TLS 1.3)
- Password hashing (Argon2id)
- Rate limiting
- CSRF protection
- SQL injection prevention
- XSS prevention
- Regular security audits
- GDPR compliance considerations
- SOC 2 readiness

### 6.5 Multi-Tenancy Isolation

- Row-Level Security (RLS) in PostgreSQL
- Tenant ID in all queries
- Separate encryption keys per tenant
- Audit logging per tenant
- Rate limiting per tenant

---

## 7. Database Schema Strategy

### 7.1 Multi-Tenant Approach

```sql
-- All tables include tenant_id
CREATE TABLE tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    -- other columns
);

-- Row Level Security
ALTER TABLE tickets ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON tickets
    USING (tenant_id = current_setting('app.current_tenant')::UUID);
```

### 7.2 Single-Tenant Approach

```sql
-- tenant_id column exists but is always the same value
-- RLS is disabled for performance
-- Simpler queries without tenant filtering
```

---

## 8. API Design Principles

### 8.1 Standards

- RESTful API design
- JSON:API specification for responses
- Consistent error format
- Pagination (cursor-based)
- Filtering and sorting
- Field selection (sparse fieldsets)
- Relationship inclusion

### 8.2 Authentication

- JWT tokens for session auth
- API keys for service auth
- OAuth2 for third-party
- Token refresh mechanism

### 8.3 Versioning

- URL-based versioning (/api/v1/)
- Deprecation headers
- Changelog maintenance

---

## 9. Success Criteria

### 9.1 Phase 1 Completion

- [ ] All P0 features implemented and tested
- [ ] All P1 features implemented and tested
- [ ] Multi-tenant deployment working
- [ ] Single-tenant deployment working
- [ ] Client portal functional
- [ ] Tactical RMM integration working
- [ ] Payment gateway integration working
- [ ] Documentation complete

### 9.2 Quality Metrics

- Code coverage > 80%
- No critical security vulnerabilities
- API response times within targets
- Successful load testing

---

## 10. Glossary

| Term | Definition |
|------|------------|
| MSP | Managed Service Provider |
| PSA | Professional Services Automation |
| RMM | Remote Monitoring and Management |
| CMDB | Configuration Management Database |
| SLA | Service Level Agreement |
| RBAC | Role-Based Access Control |
| RLS | Row-Level Security |
| SSO | Single Sign-On |
| SAML | Security Assertion Markup Language |
| OIDC | OpenID Connect |

---

**Document Control**

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | January 2026 | Claude | Initial draft |
