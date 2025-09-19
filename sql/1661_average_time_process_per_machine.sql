SELECT
	sa.machine_id,
	ROUND(AVG(ea.timestamp - sa.timestamp), 3) AS processing_time
FROM Activity as sa
INNER JOIN Activity as ea ON
	sa.machine_id = ea.machine_id AND 
	sa.process_id = ea.process_id
WHERE sa.activity_type = 'start' AND
	ea.activity_type = 'end'
GROUP BY 1;

