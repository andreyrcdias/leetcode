-- Courses
-- +-------------+---------+
-- | Column Name | Type    |
-- +-------------+---------+
-- | student     | varchar |
-- | class       | varchar |
-- +-------------+---------+

SELECT class
FROM Courses
GROUP BY 1
HAVING COUNT(*) >= 5;

