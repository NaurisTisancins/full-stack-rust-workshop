CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- CREATE TABLE IF NOT EXISTS films
-- (
--     id uuid DEFAULT uuid_generate_v1() NOT NULL CONSTRAINT films_pkey PRIMARY KEY,
--     title text NOT NULL,
--     director text NOT NULL,
--     year smallint NOT NULL,
--     poster text NOT NULL,
--     created_at timestamp with time zone default CURRENT_TIMESTAMP,
--     updated_at timestamp with time zone
-- );

-- Table for individual exercises
CREATE TABLE IF NOT EXISTS Exercises (
    exercise_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    exercise_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- Table for training days, linking exercises to specific days
CREATE TABLE IF NOT EXISTS TrainingDays (
    day_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    day_name VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- Junction table to represent the relationship between exercises and training days
CREATE TABLE IF NOT EXISTS ExerciseTrainingDayLink (
    link_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    day_id UUID REFERENCES TrainingDays(day_id),
    exercise_id UUID REFERENCES Exercises(exercise_id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- Table for training plans, linking routines to training days
CREATE TABLE IF NOT EXISTS TrainingPlans (
    training_plan_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    day_id UUID REFERENCES TrainingDays(day_id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- DROP TABLE IF EXISTS Routines;
-- Table for routines
CREATE TABLE IF NOT EXISTS Routines (
    routine_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(1000),
    training_plan_id UUID REFERENCES TrainingPlans(training_plan_id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);





