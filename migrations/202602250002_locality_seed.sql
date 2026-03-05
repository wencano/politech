insert into locality (id, code, name, locality_type, parent_id)
values
    ('00000000-0000-0000-0000-000000000001', 'PH-NCR', 'National Capital Region', 'region', null),
    ('00000000-0000-0000-0000-000000000002', 'PH-NCR-MNL', 'City of Manila', 'city', '00000000-0000-0000-0000-000000000001'),
    ('00000000-0000-0000-0000-000000000003', 'PH-NCR-MNL-D1', 'Manila District 1', 'district', '00000000-0000-0000-0000-000000000002'),
    ('00000000-0000-0000-0000-000000000004', 'PH-NCR-MNL-BRGY699', 'Barangay 699', 'barangay', '00000000-0000-0000-0000-000000000003')
on conflict (code) do nothing;
