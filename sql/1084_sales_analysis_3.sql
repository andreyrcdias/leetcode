SELECT p.product_id, p.product_name
FROM Product p 
INNER JOIN Sales s ON s.product_id = p.product_id
GROUP BY 1, 2
HAVING SUM(
	s.sale_date < '2019-01-01' OR
	s.sale_date > '2019-03-31'
) = 0

