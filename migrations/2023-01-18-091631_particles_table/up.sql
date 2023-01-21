-- Your SQL goes here
--- Table
CREATE TABLE particles (
    part_id SERIAL NOT NULL PRIMARY KEY,
    part_type VARCHAR NOT NULL,
    part_name VARCHAR NOT NULL,
    mass BIGINT NOT NULL,
    charge VARCHAR NOT NULL,
    spin VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

--- Generate Data
INSERT INTO particles ( part_type, part_name,  mass, charge, spin, created_at, updated_at) VALUES 
( 'quark', 'up', 2200000, '2/3', '1/2', current_timestamp, current_timestamp),
( 'quark', 'down', 4700000, '-1/3', '1/2', current_timestamp, current_timestamp),
( 'quark', 'top', 173100000000, '2/3', '1/2', current_timestamp, current_timestamp),
( 'quark', 'bottom', 4180000000, '-1/3', '1/2', current_timestamp, current_timestamp),
( 'quark', 'charm', 1280000000, '2/3', '1/2', current_timestamp, current_timestamp),
( 'quark', 'strange', 96000000, '-1/3', '1/2', current_timestamp, current_timestamp),
( 'lepton', 'electron', 511000, '-1', '1/2', current_timestamp, current_timestamp),
( 'lepton', 'electron neutrino', 1, '0', '1/2', current_timestamp, current_timestamp),
( 'lepton', 'muon', 105660000, '-1', '1/2', current_timestamp, current_timestamp),
( 'lepton', 'muon neutrino', 170000, '0', '1/2', current_timestamp, current_timestamp),
( 'lepton', 'tau', 1776800000, '-1', '1/2', current_timestamp, current_timestamp),
( 'lepton', 'tau neutrino', 18200000, '0', '1/2', current_timestamp, current_timestamp),
( 'gaugeBoson', 'gluon', 0, '0', '1', current_timestamp, current_timestamp),
( 'gaugeBoson', 'photon', 0, '0', '1', current_timestamp, current_timestamp),
( 'gaugeBoson', 'z boson', 91190000000, '0', '1', current_timestamp, current_timestamp),
( 'gaugeBoson', 'w boson', 80433000000, '+-1', '1', current_timestamp, current_timestamp),
( 'scalarBoson', 'higs boson', 124970000000, '0', '0', current_timestamp, current_timestamp);