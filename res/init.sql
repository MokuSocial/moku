CREATE TABLE IF NOT EXISTS ingredients (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE,
	-- identifier TEXT UNIQUE NOT NULL, 
	-- wikidata TEXT,
	-- cost_per_unit REAL,
	unit TEXT CHECK (unit IN ('kg', 'g', 'l', 'ml', 'piece'))  -- Unità di misura
);

CREATE TABLE IF NOT EXISTS recipe_ingredients (
	recipe_id INTEGER NOT NULL,
	ingredient_id INTEGER NOT NULL,
	quantity REAL NOT NULL,
	unit TEXT CHECK (unit IN ('kg', 'g', 'l', 'ml', 'piece')) NOT NULL,
	FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
	FOREIGN KEY(ingredient_id) REFERENCES ingredients(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS recipes (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	author TEXT NOT NULL,  -- Identificativo utente come stringa
	title TEXT NOT NULL,
	banner_image_url TEXT,
	-- introduction TEXT NOT NULL,
	-- conclusion TEXT NOT NULL,
	created_at INTEGER NOT NULL,
	last_updated INTEGER DEFAULT NULL,
	FOREIGN KEY(author) REFERENCES users(username) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS recipe_steps (
	recipe_id INTEGER NOT NULL,
	step_number INTEGER NOT NULL,
	description TEXT NOT NULL,
	image_url TEXT,
	PRIMARY KEY(recipe_id, step_number),
	FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users (
	username TEXT PRIMARY KEY NOT NULL,  -- Identificatore dell'utente (stringa)
	password_hash TEXT NOT NULL,
	email TEXT UNIQUE NOT NULL,
	created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS tag (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	text TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS recipe_tags (
	recipe_id INTEGER NOT NULL,
	tag_id INTEGER NOT NULL,
	FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
	FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS indications (
	recipe_id INTEGER NOT NULL,
	tag_id INTEGER NOT NULL,
	value TEXT NOT NULL,
	FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
	FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
