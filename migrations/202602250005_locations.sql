-- Locations: Philippine regions (and later provinces/municipalities). Match GeoJSON features via code or name.
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

-- Philippine regions (national level). geojson_key matches feature.properties.adm1_en in country.0.001.json.
insert into location (code, name, level, geojson_key, traits) values
    ('PH-NCR', 'National Capital Region', 'region', 'National Capital Region (NCR)', '{"urban": 9, "population": 8, "economic": 8}'::jsonb),
    ('PH-CAR', 'Cordillera Administrative Region', 'region', 'Cordillera Administrative Region (CAR)', '{"rural": 6, "agriculture": 5, "tourism": 6}'::jsonb),
    ('PH-01', 'Ilocos Region', 'region', 'Region I (Ilocos Region)', '{"agriculture": 6, "rural": 7}'::jsonb),
    ('PH-02', 'Cagayan Valley', 'region', 'Region II (Cagayan Valley)', '{"agriculture": 8, "rural": 8}'::jsonb),
    ('PH-03', 'Central Luzon', 'region', 'Region III (Central Luzon)', '{"agriculture": 6, "urban": 5, "economic": 6}'::jsonb),
    ('PH-4A', 'Calabarzon', 'region', 'Region IV-A (CALABARZON)', '{"urban": 7, "economic": 7, "industrial": 7}'::jsonb),
    ('PH-4B', 'Mimaropa', 'region', 'MIMAROPA Region', '{"tourism": 7, "rural": 7}'::jsonb),
    ('PH-05', 'Bicol Region', 'region', 'Region V (Bicol Region)', '{"agriculture": 6, "rural": 7, "disaster": 7}'::jsonb),
    ('PH-06', 'Western Visayas', 'region', 'Region VI (Western Visayas)', '{"agriculture": 6, "urban": 4}'::jsonb),
    ('PH-07', 'Central Visayas', 'region', 'Region VII (Central Visayas)', '{"tourism": 8, "urban": 5}'::jsonb),
    ('PH-08', 'Eastern Visayas', 'region', 'Region VIII (Eastern Visayas)', '{"rural": 7, "agriculture": 6, "disaster": 6}'::jsonb),
    ('PH-09', 'Zamboanga Peninsula', 'region', 'Region IX (Zamboanga Peninsula)', '{"agriculture": 5, "rural": 6}'::jsonb),
    ('PH-10', 'Northern Mindanao', 'region', 'Region X (Northern Mindanao)', '{"agriculture": 7, "economic": 5}'::jsonb),
    ('PH-11', 'Davao Region', 'region', 'Region XI (Davao Region)', '{"urban": 5, "agriculture": 6, "economic": 6}'::jsonb),
    ('PH-12', 'Soccsksargen', 'region', 'Region XII (SOCCSKSARGEN)', '{"agriculture": 8, "rural": 7}'::jsonb),
    ('PH-13', 'Caraga', 'region', 'Region XIII (Caraga)', '{"agriculture": 6, "rural": 7, "mining": 5}'::jsonb),
    ('PH-BARMM', 'Bangsamoro', 'region', 'Bangsamoro Autonomous Region In Muslim Mindanao (BARMM)', '{"rural": 6, "agriculture": 5}'::jsonb)
on conflict (code) do update set
    name = excluded.name,
    geojson_key = excluded.geojson_key,
    traits = excluded.traits;
