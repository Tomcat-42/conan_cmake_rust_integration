#include <deep_thought/answer.hpp>

#include <chrono>
#include <iostream>
#include <thread>

auto main() -> int {
  auto task = []() {
    std::cout << "Thinking..." << std::endl;
    std::this_thread::sleep_for(std::chrono::seconds(10));
    std::cout << "The answer is " << deep_thought::answer() << std::endl;
  };

  std::thread thread(task);
  thread.join();
  return 0;
}
