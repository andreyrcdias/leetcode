SELECT
	u.name,
	SUM(t.amount) AS balance
FROM Users u
INNER JOIN Transactions t ON t.account = u.account
GROUP BY 1
HAVING balance > 10000;

