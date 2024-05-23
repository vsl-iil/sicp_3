// Чайник
class Teapot {
public:
  bool filled;
  bool boiling;

  Teapot();

private:
  
};

Teapot::Teapot() {
}

// Спички
class Matches { 
public: 
  int amount_of_matches;

  virtual void light_match();
};

// Кран
class Tap { 
public: 
  bool opened; 

  virtual void open_tap();
};

// Плита
class Stove { 
public: 
  bool lit; 

  virtual void light_stove(Matches matches);
};

// Класс, определяющий процесс кипения чайника
class BoilingTeapot {
public:
  BoilingTeapot();

/* Внутрь залезть нельзя */
private:
  Matches matches;
  Tap tap;
  Stove stove;

  BoilingTeapot(Matches, Tap, Stove);
};

BoilingTeapot::BoilingTeapot(Matches _matches, Tap _tap, Stove _stove) {
  this->matches = _matches;
  this->tap = _tap;
  this->stove = _stove;
}


int main (int argc, char *argv[]) {
  
  return 0;
}
