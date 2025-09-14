SELECT
	b.book_id,
	b.title,
	b.author,
	b.genre,
	b.pages,
	(MAX(rs.session_rating) - MIN(rs.session_rating)) AS rating_spread,
	ROUND((SUM(rs.session_rating <= 2) + SUM(rs.session_rating >= 4)) / COUNT(1), 2) AS polarization_score
FROM books b
JOIN reading_sessions rs ON b.book_id = rs.book_id
GROUP BY b.book_id
HAVING
	COUNT(1) >= 5 AND
	MAX(rs.session_rating) >= 4 AND
	MIN(rs.session_rating) <= 2 AND
	polarization_score >= 0.6
ORDER BY polarization_score DESC, b.title DESC

