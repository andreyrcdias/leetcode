-- +--------------+---------+
-- | Column Name  | Type    |
-- +--------------+---------+
-- | player_id    | int     |
-- | device_id    | int     |
-- | event_date   | date    |
-- | games_played | int     |
-- +--------------+---------+

SELECT
	player_id,
	event_date AS first_login
FROM activity
GROUP BY player_id
ORDER BY event_date

