-- Employee
-- +-------------+---------+
-- | Column Name | Type    |
-- +-------------+---------+
-- | id          | int     |
-- | name        | varchar |
-- | salary      | int     |
-- | managerId   | int     |
-- +-------------+---------+

SELECT 
    name AS Employee
FROM 
    Employee worker
WHERE 
    worker.salary > (
        SELECT salary 
        FROM Employee manager 
        WHERE manager.id = worker.managerId
    )
