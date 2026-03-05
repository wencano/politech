-- Locations: Philippine regions. Match GeoJSON via geojson_key.
-- traits: HEXACO (0-1). Urban regions: high extraversion/openness; rural: high honesty_humility/agreeableness, low extraversion/openness.
create table if not exists location (
    id uuid primary key default gen_random_uuid(),
    code text not null unique,
    name text not null,
    level text not null default 'region',
    geojson_key text,
    traits jsonb not null default '{}',
    created_at timestamptz not null default now()
);

create index if not exists idx_location_code on location(code);
create index if not exists idx_location_level on location(level);

-- Urban (NCR, Calabarzon, Central Visayas, Davao): high X, O, C; lower H-H, A. Rural: high H-H, A; low X, O.
insert into location (code, name, level, geojson_key, traits) values
    ('PH-NCR', 'National Capital Region', 'region', 'National Capital Region (NCR)', '{"honesty_humility": 0.25, "emotionality": 0.35, "extraversion": 0.9, "agreeableness": 0.3, "conscientiousness": 0.85, "openness": 0.9}'::jsonb),
    ('PH-CAR', 'Cordillera Administrative Region', 'region', 'Cordillera Administrative Region (CAR)', '{"honesty_humility": 0.88, "emotionality": 0.6, "extraversion": 0.2, "agreeableness": 0.82, "conscientiousness": 0.7, "openness": 0.25}'::jsonb),
    ('PH-01', 'Ilocos Region', 'region', 'Region I (Ilocos Region)', '{"honesty_humility": 0.82, "emotionality": 0.55, "extraversion": 0.22, "agreeableness": 0.78, "conscientiousness": 0.72, "openness": 0.22}'::jsonb),
    ('PH-02', 'Cagayan Valley', 'region', 'Region II (Cagayan Valley)', '{"honesty_humility": 0.85, "emotionality": 0.58, "extraversion": 0.2, "agreeableness": 0.8, "conscientiousness": 0.68, "openness": 0.25}'::jsonb),
    ('PH-03', 'Central Luzon', 'region', 'Region III (Central Luzon)', '{"honesty_humility": 0.5, "emotionality": 0.5, "extraversion": 0.55, "agreeableness": 0.5, "conscientiousness": 0.65, "openness": 0.55}'::jsonb),
    ('PH-4A', 'Calabarzon', 'region', 'Region IV-A (CALABARZON)', '{"honesty_humility": 0.3, "emotionality": 0.4, "extraversion": 0.85, "agreeableness": 0.35, "conscientiousness": 0.8, "openness": 0.85}'::jsonb),
    ('PH-4B', 'Mimaropa', 'region', 'MIMAROPA Region', '{"honesty_humility": 0.8, "emotionality": 0.6, "extraversion": 0.25, "agreeableness": 0.8, "conscientiousness": 0.62, "openness": 0.28}'::jsonb),
    ('PH-05', 'Bicol Region', 'region', 'Region V (Bicol Region)', '{"honesty_humility": 0.78, "emotionality": 0.62, "extraversion": 0.28, "agreeableness": 0.78, "conscientiousness": 0.6, "openness": 0.3}'::jsonb),
    ('PH-06', 'Western Visayas', 'region', 'Region VI (Western Visayas)', '{"honesty_humility": 0.58, "emotionality": 0.52, "extraversion": 0.45, "agreeableness": 0.6, "conscientiousness": 0.58, "openness": 0.48}'::jsonb),
    ('PH-07', 'Central Visayas', 'region', 'Region VII (Central Visayas)', '{"honesty_humility": 0.38, "emotionality": 0.42, "extraversion": 0.72, "agreeableness": 0.42, "conscientiousness": 0.68, "openness": 0.7}'::jsonb),
    ('PH-08', 'Eastern Visayas', 'region', 'Region VIII (Eastern Visayas)', '{"honesty_humility": 0.82, "emotionality": 0.58, "extraversion": 0.22, "agreeableness": 0.8, "conscientiousness": 0.65, "openness": 0.25}'::jsonb),
    ('PH-09', 'Zamboanga Peninsula', 'region', 'Region IX (Zamboanga Peninsula)', '{"honesty_humility": 0.6, "emotionality": 0.5, "extraversion": 0.4, "agreeableness": 0.62, "conscientiousness": 0.6, "openness": 0.45}'::jsonb),
    ('PH-10', 'Northern Mindanao', 'region', 'Region X (Northern Mindanao)', '{"honesty_humility": 0.52, "emotionality": 0.48, "extraversion": 0.5, "agreeableness": 0.58, "conscientiousness": 0.68, "openness": 0.52}'::jsonb),
    ('PH-11', 'Davao Region', 'region', 'Region XI (Davao Region)', '{"honesty_humility": 0.35, "emotionality": 0.45, "extraversion": 0.75, "agreeableness": 0.4, "conscientiousness": 0.7, "openness": 0.72}'::jsonb),
    ('PH-12', 'Soccsksargen', 'region', 'Region XII (SOCCSKSARGEN)', '{"honesty_humility": 0.85, "emotionality": 0.52, "extraversion": 0.25, "agreeableness": 0.78, "conscientiousness": 0.7, "openness": 0.25}'::jsonb),
    ('PH-13', 'Caraga', 'region', 'Region XIII (Caraga)', '{"honesty_humility": 0.8, "emotionality": 0.55, "extraversion": 0.25, "agreeableness": 0.76, "conscientiousness": 0.65, "openness": 0.28}'::jsonb),
    ('PH-BARMM', 'Bangsamoro', 'region', 'Bangsamoro Autonomous Region In Muslim Mindanao (BARMM)', '{"honesty_humility": 0.82, "emotionality": 0.6, "extraversion": 0.22, "agreeableness": 0.82, "conscientiousness": 0.6, "openness": 0.25}'::jsonb)
on conflict (code) do update set
    name = excluded.name,
    geojson_key = excluded.geojson_key,
    traits = excluded.traits;
