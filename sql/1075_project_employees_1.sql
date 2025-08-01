SELECT
	p.project_id,
	ROUND(AVG(E.experience_years), 2) AS average_years
FROM Project p 
JOIN Employee e ON e.employee_id = p.employee_id
GROUP BY 1;

