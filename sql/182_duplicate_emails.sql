-- Person
-- +-------------+---------+
-- | Column Name | Type    |
-- +-------------+---------+
-- | id          | int     |
-- | email       | varchar |
-- +-------------+---------+

SELECT email AS Email
FROM Person
GROUP BY Email
HAVING COUNT(*) > 1;

