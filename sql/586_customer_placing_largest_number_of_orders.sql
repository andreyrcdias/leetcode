-- Orders
-- +-----------------+----------+
-- | Column Name     | Type     |
-- +-----------------+----------+
-- | order_number    | int      |
-- | customer_number | int      |
-- +-----------------+----------+

SELECT customer_number
FROM Orders
GROUP BY 1
ORDER BY COUNT(*) DESC
LIMIT 1;

