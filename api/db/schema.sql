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

-- DROP TABLE IF EXISTS Routines CASCADE;
-- Table for routines
CREATE TABLE IF NOT EXISTS Routines (
    routine_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(1000),
    is_active BOOLEAN DEFAULT FALSE,
    disabled BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- DROP TABLE IF EXISTS TrainingDays CASCADE;
-- Table for training days, linking exercises to specific days
CREATE TABLE IF NOT EXISTS TrainingDays (
    day_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    routine_id UUID REFERENCES Routines(routine_id) ON DELETE CASCADE,
    day_name VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- DROP TABLE IF EXISTS Exercises CASCADE;
-- Table for individual exercises
CREATE TABLE IF NOT EXISTS Exercises (
    exercise_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    exercise_name VARCHAR(255) UNIQUE NOT NULL,
    exercise_description VARCHAR(1000),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- DROP TABLE IF EXISTS ExerciseTrainingDayLink CASCADE;
-- Junction table to represent the relationship between exercises and training days
CREATE TABLE IF NOT EXISTS ExerciseTrainingDayLink (
    link_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    exercise_id UUID REFERENCES Exercises(exercise_id),
    day_id UUID REFERENCES TrainingDays(day_id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- DROP TABLE IF EXISTS Sessions CASCADE;
-- Table for individual sessions
CREATE TABLE IF NOT EXISTS Sessions (
    session_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    day_id UUID REFERENCES TrainingDays(day_id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

DROP TABLE IF EXISTS SessionExercises CASCADE;


-- DROP TABLE IF EXISTS SessionExercisePerformace CASCADE;
-- Table for individual session exercise performance
CREATE TABLE IF NOT EXISTS SessionExercisePerformance (
    performance_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    session_id UUID REFERENCES Sessions(session_id),
    exercise_id UUID REFERENCES Exercises(exercise_id),
    weight DECIMAL(5,2),
    reps SMALLINT,
    set SMALLINT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);






