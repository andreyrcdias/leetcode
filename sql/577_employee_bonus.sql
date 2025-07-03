-- Employee
-- +-------------+---------+
-- | Column Name | Type    |
-- +-------------+---------+
-- | empId       | int     |
-- | name        | varchar |
-- | supervisor  | int     |
-- | salary      | int     |
-- +-------------+---------+

-- Bonus
-- +-------------+------+
-- | Column Name | Type |
-- +-------------+------+
-- | empId       | int  |
-- | bonus       | int  |
-- +-------------+------+

SELECT
	e.name,
	b.bonus
FROM Employee e
LEFT JOIN Bonus b ON b.empId = e.empId
WHERE IFNULL(b.bonus, 0) < 1000;

