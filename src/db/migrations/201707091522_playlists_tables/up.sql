CREATE TABLE playlists (
	id INTEGER PRIMARY KEY NOT NULL,
	owner INTEGER NOT NULL,
	name TEXT NOT NULL,
	FOREIGN KEY(owner) REFERENCES users(id) ON DELETE CASCADE,
	UNIQUE(owner, name) ON CONFLICT REPLACE
);

CREATE TABLE playlist_songs (
	id INTEGER PRIMARY KEY NOT NULL,
	playlist INTEGER NOT NULL,
	path TEXT NOT NULL,
	ordering INTEGER NOT NULL,
	FOREIGN KEY(path) REFERENCES songs(path),
	FOREIGN KEY(playlist) REFERENCES playlists(id) ON DELETE CASCADE ON UPDATE CASCADE,
	UNIQUE(playlist, ordering) ON CONFLICT REPLACE
);
