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
    day_name VARCHAR(50) NOT NULL,
    in_progress BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- DROP TABLE IF EXISTS SessionExercises CASCADE;


-- DROP TABLE IF EXISTS SessionExercisePerformance CASCADE;
-- Table for individual session exercise performance
-- CREATE TABLE IF NOT EXISTS SessionExercisePerformance (
--     performance_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
--     session_id UUID REFERENCES Sessions(session_id),
--     exercise_id UUID REFERENCES Exercises(exercise_id),
--     weight FLOAT4,
--     reps SMALLINT,
--     set_number SMALLINT,
--     rir SMALLINT,
--     created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMP WITH TIME ZONE,
--     CONSTRAINT unique_set_number UNIQUE (session_id, exercise_id, set_number)
-- );

CREATE OR REPLACE FUNCTION update_set_numbers()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE SessionExercisePerformance
    SET set_number = set_number - 1
    WHERE session_id = OLD.session_id
    AND exercise_id = OLD.exercise_id
    AND set_number > OLD.set_number;

    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS SessionExercisePerformance (
    performance_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    session_id UUID REFERENCES Sessions(session_id),
    exercise_id UUID REFERENCES Exercises(exercise_id),
    weight FLOAT4,
    reps SMALLINT,
    set_number SMALLINT,
    rir SMALLINT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT unique_set_number UNIQUE (session_id, exercise_id, set_number)
);

DROP TRIGGER IF EXISTS update_set_numbers_trigger ON SessionExercisePerformance;

-- Create trigger
CREATE TRIGGER update_set_numbers_trigger
BEFORE DELETE ON SessionExercisePerformance
FOR EACH ROW
EXECUTE FUNCTION update_set_numbers();






