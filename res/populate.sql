-- ================================
-- DATI DI ESEMPIO
-- ================================

INSERT INTO users (username, password_hash, email, created_at) VALUES
('alice', 'hash1', 'alice@example.com', strftime('%s','now')),
('bob', 'hash2', 'bob@example.com', strftime('%s','now')),
('carol', 'hash3', 'carol@example.com', strftime('%s','now'));

INSERT INTO ingredients (name, unit) VALUES
('Pomodori pelati', 'g'),
('Spaghetti', 'g'),
('Olio d''oliva', 'ml'),
('Aglio', 'piece'),
('Basilico fresco', 'g'),
('Sale', 'g'),
('Pepe nero', 'g'),
('Parmigiano grattugiato', 'g'),
('Uova', 'piece'),
('Pancetta', 'g');

INSERT INTO tags (text) VALUES
('vegetariano'),
('italiano'),
('pasta'),
('veloce'),
('classico');

INSERT INTO recipes (author, title, banner_image_url, servings, prep_time, cook_time, rest_time, difficulty) VALUES
('alice', 'Spaghetti al pomodoro', 'https://example.com/spaghetti.jpg', 4, 20, 15, NULL, 'easy'),
('bob', 'Carbonara', 'https://example.com/carbonara.jpg', 4, 15, 10, NULL, 'medium');

INSERT INTO recipe_ingredients (recipe_id, ingredient_id, quantity, unit) VALUES
(1, 2, 200, 'g'),
(1, 1, 400, 'g'),
(1, 3, 30, 'ml'),
(1, 4, 1, 'piece'),
(1, 5, 5, 'g'),
(1, 6, 3, 'g'),
(1, 7, 1, 'g'),
(2, 2, 200, 'g'),
(2, 10, 100, 'g'),
(2, 9, 2, 'piece'),
(2, 6, 2, 'g'),
(2, 7, 1, 'g'),
(2, 8, 30, 'g');

INSERT INTO recipe_steps (recipe_id, step_number, description) VALUES
(1, 1, 'Porta a ebollizione una pentola d''acqua salata.'),
(1, 2, 'Cuoci gli spaghetti al dente.'),
(1, 3, 'In una padella, soffriggi l''aglio nell''olio.'),
(1, 4, 'Aggiungi i pomodori pelati e cuoci per 10 minuti.'),
(1, 5, 'Scola la pasta e uniscila al sugo con il basilico. Servi caldo.'),
(2, 1, 'Cuoci la pancetta in padella finché è croccante.'),
(2, 2, 'Sbatti le uova con il parmigiano, sale e pepe.'),
(2, 3, 'Cuoci la pasta e scolala al dente.'),
(2, 4, 'Unisci la pasta alla pancetta, togli dal fuoco e aggiungi il composto di uova.'),
(2, 5, 'Mescola velocemente e servi subito.');

INSERT INTO recipe_tags (recipe_id, tag_id) VALUES
(1, 1), (1, 2), (1, 3), (1, 4),
(2, 2), (2, 3), (2, 5);
