-- Triangle
-- +-------------+------+
-- | Column Name | Type |
-- +-------------+------+
-- | x           | int  |
-- | y           | int  |
-- | z           | int  |
-- +-------------+------+

SELECT 
	*,
	IF(x+y > z AND x+z > y AND y+z > x, 'Yes', 'No') AS triangle
FROM Triangle

