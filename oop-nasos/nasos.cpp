#include <iostream>

class Nasos {
public:
  Nasos(float napor, float plotn, float kpd, float delivery)
    : napor(napor), plotn(plotn), kpd(kpd), delivery(delivery) {}
  virtual ~Nasos() {}

  virtual float calc_power(float effic) = 0;

  virtual float calc_effic(float power) = 0;

protected:
    float napor;
    float plotn;
    float kpd;
    float delivery;
};


class cNasos : public Nasos {
public:
  cNasos(float napor, float plotn, float kpd, float delivery)
    : Nasos(napor, plotn, kpd, delivery) {}
  ~cNasos() override {}

  float calc_power(float effic) {
    return kpd/(this->delivery * this->napor * this->plotn/3670);
  }

  float calc_effic(float power) {
    return this->delivery * this->napor * this->plotn/3670 * power;
  }

};


int main() {
  cNasos pump(100, 500, 0.8, 10);

  float power = pump.calc_power(0.9);
  std::cout << "Power: " << power << std::endl;

  float effic = pump.calc_effic(power);
  std::cout << "Efficiency: " << effic << std::endl;

  return 0;
}
