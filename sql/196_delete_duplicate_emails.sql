-- Person
-- +-------------+---------+
-- | Column Name | Type    |
-- +-------------+---------+
-- | id          | int     |
-- | email       | varchar |
-- +-------------+---------+
DELETE p2
FROM Person p1
INNER JOIN Person p2 ON p1.email = p2.email
WHERE p1.id < p2.id;

