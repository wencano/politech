-- Topics: user-submitted topics with traits (jsonb) for alignment to locations/personas later.
create table if not exists topic (
    id uuid primary key default gen_random_uuid(),
    title text not null,
    traits jsonb not null default '{}',
    created_at timestamptz not null default now()
);

create index if not exists idx_topic_created_at on topic(created_at desc);

-- Sample topics with random traits (1-10 scale for alignment-style dimensions).
insert into topic (id, title, traits, created_at) values
    (gen_random_uuid(), 'Infrastructure and roads', '{"infrastructure": 7, "economic_growth": 6, "jobs": 5, "urban": 8}'::jsonb, now() - interval '2 days'),
    (gen_random_uuid(), 'Education quality', '{"education": 9, "youth": 8, "public_service": 7}'::jsonb, now() - interval '1 day'),
    (gen_random_uuid(), 'Health and hospitals', '{"health": 9, "rural": 6, "public_service": 8}'::jsonb, now() - interval '3 hours'),
    (gen_random_uuid(), 'Agriculture and farmers', '{"agriculture": 9, "rural": 9, "economic_growth": 5}'::jsonb, now() - interval '5 hours'),
    (gen_random_uuid(), 'Climate and disaster preparedness', '{"environment": 8, "disaster": 9, "rural": 7}'::jsonb, now() - interval '1 hour');
