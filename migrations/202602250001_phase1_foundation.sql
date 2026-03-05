create extension if not exists "uuid-ossp";

create table if not exists app_user (
    id uuid primary key,
    email text not null unique,
    password_hash text not null,
    role text not null default 'user',
    status text not null default 'active',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists user_session (
    id uuid primary key,
    user_id uuid not null references app_user(id) on delete cascade,
    status text not null default 'active',
    created_at timestamptz not null default now(),
    expires_at timestamptz not null
);

create index if not exists idx_user_session_user_id on user_session(user_id);

create table if not exists locality (
    id uuid primary key,
    code text not null unique,
    name text not null,
    locality_type text not null,
    parent_id uuid references locality(id) on delete set null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index if not exists idx_locality_parent_id on locality(parent_id);
create index if not exists idx_locality_type on locality(locality_type);
