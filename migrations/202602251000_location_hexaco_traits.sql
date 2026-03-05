-- Update location traits to HEXACO 0-1 (for alignment dot product). Does not change schema.
update location set traits = '{"honesty_humility": 0.4, "emotionality": 0.5, "extraversion": 0.8, "agreeableness": 0.5, "conscientiousness": 0.7, "openness": 0.8}'::jsonb where code = 'PH-NCR';
update location set traits = '{"honesty_humility": 0.7, "emotionality": 0.4, "extraversion": 0.4, "agreeableness": 0.6, "conscientiousness": 0.6, "openness": 0.5}'::jsonb where code = 'PH-CAR';
update location set traits = '{"honesty_humility": 0.7, "emotionality": 0.4, "extraversion": 0.4, "agreeableness": 0.6, "conscientiousness": 0.7, "openness": 0.4}'::jsonb where code = 'PH-01';
update location set traits = '{"honesty_humility": 0.6, "emotionality": 0.4, "extraversion": 0.4, "agreeableness": 0.6, "conscientiousness": 0.6, "openness": 0.5}'::jsonb where code = 'PH-02';
update location set traits = '{"honesty_humility": 0.5, "emotionality": 0.5, "extraversion": 0.5, "agreeableness": 0.5, "conscientiousness": 0.6, "openness": 0.5}'::jsonb where code = 'PH-03';
update location set traits = '{"honesty_humility": 0.4, "emotionality": 0.5, "extraversion": 0.7, "agreeableness": 0.5, "conscientiousness": 0.6, "openness": 0.6}'::jsonb where code = 'PH-4A';
update location set traits = '{"honesty_humility": 0.6, "emotionality": 0.4, "extraversion": 0.4, "agreeableness": 0.6, "conscientiousness": 0.5, "openness": 0.5}'::jsonb where code = 'PH-4B';
update location set traits = '{"honesty_humility": 0.6, "emotionality": 0.5, "extraversion": 0.5, "agreeableness": 0.6, "conscientiousness": 0.5, "openness": 0.5}'::jsonb where code = 'PH-05';
update location set traits = '{"honesty_humility": 0.5, "emotionality": 0.5, "extraversion": 0.5, "agreeableness": 0.6, "conscientiousness": 0.5, "openness": 0.5}'::jsonb where code = 'PH-06';
update location set traits = '{"honesty_humility": 0.5, "emotionality": 0.5, "extraversion": 0.6, "agreeableness": 0.5, "conscientiousness": 0.5, "openness": 0.6}'::jsonb where code = 'PH-07';
update location set traits = '{"honesty_humility": 0.6, "emotionality": 0.5, "extraversion": 0.4, "agreeableness": 0.6, "conscientiousness": 0.5, "openness": 0.4}'::jsonb where code = 'PH-08';
update location set traits = '{"honesty_humility": 0.5, "emotionality": 0.5, "extraversion": 0.5, "agreeableness": 0.5, "conscientiousness": 0.5, "openness": 0.5}'::jsonb where code = 'PH-09';
update location set traits = '{"honesty_humility": 0.5, "emotionality": 0.5, "extraversion": 0.5, "agreeableness": 0.5, "conscientiousness": 0.6, "openness": 0.5}'::jsonb where code = 'PH-10';
update location set traits = '{"honesty_humility": 0.5, "emotionality": 0.5, "extraversion": 0.6, "agreeableness": 0.5, "conscientiousness": 0.5, "openness": 0.5}'::jsonb where code = 'PH-11';
update location set traits = '{"honesty_humility": 0.6, "emotionality": 0.4, "extraversion": 0.4, "agreeableness": 0.6, "conscientiousness": 0.6, "openness": 0.4}'::jsonb where code = 'PH-12';
update location set traits = '{"honesty_humility": 0.5, "emotionality": 0.5, "extraversion": 0.4, "agreeableness": 0.5, "conscientiousness": 0.5, "openness": 0.5}'::jsonb where code = 'PH-13';
update location set traits = '{"honesty_humility": 0.6, "emotionality": 0.5, "extraversion": 0.4, "agreeableness": 0.6, "conscientiousness": 0.5, "openness": 0.4}'::jsonb where code = 'PH-BARMM';
