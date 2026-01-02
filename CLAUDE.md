# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PSA Platform is a fullstack Professional Services Automation application for Managed Service Providers (MSPs). Built with Dioxus (Rust fullstack framework), it combines frontend (WASM) and backend in a single codebase with multi-tenant SaaS or single-tenant self-hosted deployment options.

## Build & Development Commands

```bash
# Install dependencies
npm install

# Development (starts Tailwind watcher + Dioxus dev server on http://localhost:8080)
npm run dev

# Production build (optimized release binary)
npm run build

# Build CSS only
npm run build:css

# Watch CSS only
npm run watch:css
```

## Database

- PostgreSQL 18 required
- Migrations in `migrations/` run automatically on startup when `RUN_MIGRATIONS=true`
- Connection string: `DATABASE_URL` in `.env`

## Feature Flags (Cargo.toml)

Compile-time features control deployment mode and compilation targets:

- `multi-tenant` (default) - SaaS with tenant isolation
- `single-tenant` - Self-hosted single instance
- `server` - Backend compilation (axum, sqlx, etc.)
- `web` - Frontend WASM compilation
- `fullstack` - Both server and web

Check at runtime: `AppConfig::is_multi_tenant()`

## Architecture

**Dioxus Fullstack Pattern:**
- Single codebase compiles to both server (native) and client (WASM)
- Server functions use `#[server]` attribute for RPC-style calls from client
- Pages in `src/pages/` are Dioxus components with routing
- Components in `src/components/` are reusable UI elements
- Hooks in `src/hooks/` provide reactive state management

**Module Structure (`src/modules/`):**
Each business domain is a self-contained module with its own models, handlers, and logic:
- `auth/` - JWT authentication, password hashing (argon2), role-based access
- `tenants/` - Multi-tenant isolation and configuration
- `tickets/` - Service desk ticketing with status tracking
- `time_tracking/` - Billable time entries and timesheets
- `projects/` - Project and task management
- `contracts/` - Contract lifecycle management
- `billing/` - Invoicing and payments (Stripe integration)
- `assets/` - CMDB and asset tracking
- `knowledge_base/` - Markdown articles with pulldown-cmark
- `notifications/` - Multi-channel (email via lettre, SMS via Twilio, Slack, Teams)
- `rmm/` - Tactical RMM integration
- `portal/` - Client-facing portal (limited ticket/invoice access)
- `sla/` - SLA definitions and violation tracking
- `audit/` - Change logging

**Database Layer (`src/db/`):**
- SQLx with compile-time query verification against PostgreSQL
- Async connection pooling

**API Layer (`src/api/`):**
- Axum routes and handlers
- Tower middleware for CORS, tracing, compression
- Rate limiting with governor

## Key Dependencies

- **dioxus 0.7.2** - Fullstack reactive UI framework
- **axum 0.8** - HTTP server (WebSocket, multipart)
- **sqlx 0.8** - Async PostgreSQL with compile-time checks
- **tokio** - Async runtime
- **jsonwebtoken** - JWT handling
- **argon2** - Password hashing
- **lettre** - SMTP email
- **moka** - Caching
- **minijinja** - Templating

## Styling

Tailwind CSS 4.0 with:
- Custom primary color palette
- Dark mode (class strategy)
- Inter and JetBrains Mono fonts
- Custom animations in `tailwind.config.js`

CSS is compiled from `input.css` to `assets/styles.css`.

## Environment Configuration

Copy `.env.example` to `.env`. Key variables:
- `DATABASE_URL` - PostgreSQL connection
- `JWT_SECRET` - Token signing key
- `ENCRYPTION_KEY` - 32-byte encryption key
- `STRIPE_*` - Payment processing
- `TACTICAL_RMM_*` - RMM integration
- `TWILIO_*` - SMS notifications
- `SLACK_*`, `TEAMS_*` - Chat integrations
- `SMTP_*` - Email configuration
