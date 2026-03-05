-- Apply urban/rural varied HEXACO so alignment chart is not homogeneous.
-- Urban (NCR, Calabarzon, Central Visayas, Davao): high extraversion, openness; lower H-H, agreeableness.
-- Rural (CAR, Ilocos, Cagayan Valley, Mimaropa, Bicol, Eastern Visayas, Soccsksargen, Caraga, BARMM): high H-H, agreeableness; low extraversion, openness.
update location set traits = '{"honesty_humility": 0.25, "emotionality": 0.35, "extraversion": 0.9, "agreeableness": 0.3, "conscientiousness": 0.85, "openness": 0.9}'::jsonb where code = 'PH-NCR';
update location set traits = '{"honesty_humility": 0.88, "emotionality": 0.6, "extraversion": 0.2, "agreeableness": 0.82, "conscientiousness": 0.7, "openness": 0.25}'::jsonb where code = 'PH-CAR';
update location set traits = '{"honesty_humility": 0.82, "emotionality": 0.55, "extraversion": 0.22, "agreeableness": 0.78, "conscientiousness": 0.72, "openness": 0.22}'::jsonb where code = 'PH-01';
update location set traits = '{"honesty_humility": 0.85, "emotionality": 0.58, "extraversion": 0.2, "agreeableness": 0.8, "conscientiousness": 0.68, "openness": 0.25}'::jsonb where code = 'PH-02';
update location set traits = '{"honesty_humility": 0.5, "emotionality": 0.5, "extraversion": 0.55, "agreeableness": 0.5, "conscientiousness": 0.65, "openness": 0.55}'::jsonb where code = 'PH-03';
update location set traits = '{"honesty_humility": 0.3, "emotionality": 0.4, "extraversion": 0.85, "agreeableness": 0.35, "conscientiousness": 0.8, "openness": 0.85}'::jsonb where code = 'PH-4A';
update location set traits = '{"honesty_humility": 0.8, "emotionality": 0.6, "extraversion": 0.25, "agreeableness": 0.8, "conscientiousness": 0.62, "openness": 0.28}'::jsonb where code = 'PH-4B';
update location set traits = '{"honesty_humility": 0.78, "emotionality": 0.62, "extraversion": 0.28, "agreeableness": 0.78, "conscientiousness": 0.6, "openness": 0.3}'::jsonb where code = 'PH-05';
update location set traits = '{"honesty_humility": 0.58, "emotionality": 0.52, "extraversion": 0.45, "agreeableness": 0.6, "conscientiousness": 0.58, "openness": 0.48}'::jsonb where code = 'PH-06';
update location set traits = '{"honesty_humility": 0.38, "emotionality": 0.42, "extraversion": 0.72, "agreeableness": 0.42, "conscientiousness": 0.68, "openness": 0.7}'::jsonb where code = 'PH-07';
update location set traits = '{"honesty_humility": 0.82, "emotionality": 0.58, "extraversion": 0.22, "agreeableness": 0.8, "conscientiousness": 0.65, "openness": 0.25}'::jsonb where code = 'PH-08';
update location set traits = '{"honesty_humility": 0.6, "emotionality": 0.5, "extraversion": 0.4, "agreeableness": 0.62, "conscientiousness": 0.6, "openness": 0.45}'::jsonb where code = 'PH-09';
update location set traits = '{"honesty_humility": 0.52, "emotionality": 0.48, "extraversion": 0.5, "agreeableness": 0.58, "conscientiousness": 0.68, "openness": 0.52}'::jsonb where code = 'PH-10';
update location set traits = '{"honesty_humility": 0.35, "emotionality": 0.45, "extraversion": 0.75, "agreeableness": 0.4, "conscientiousness": 0.7, "openness": 0.72}'::jsonb where code = 'PH-11';
update location set traits = '{"honesty_humility": 0.85, "emotionality": 0.52, "extraversion": 0.25, "agreeableness": 0.78, "conscientiousness": 0.7, "openness": 0.25}'::jsonb where code = 'PH-12';
update location set traits = '{"honesty_humility": 0.8, "emotionality": 0.55, "extraversion": 0.25, "agreeableness": 0.76, "conscientiousness": 0.65, "openness": 0.28}'::jsonb where code = 'PH-13';
update location set traits = '{"honesty_humility": 0.82, "emotionality": 0.6, "extraversion": 0.22, "agreeableness": 0.82, "conscientiousness": 0.6, "openness": 0.25}'::jsonb where code = 'PH-BARMM';
