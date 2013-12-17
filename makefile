
SOURCES = physics.rs
EXE = physics
CC = rustc

test: $(SOURCES) particle.rs element.rs unit.rs
	$(CC) --test $(SOURCES) -o $(EXE)
	./$(EXE)

clean:
	rm $(EXE)

