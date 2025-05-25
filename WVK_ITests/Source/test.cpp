#include "tests.h"

::testing::Environment* g_vk_env = nullptr;

GTEST_API_ int main(int argc, char** argv) {
	printf("Running main() from %s\n", __FILE__);
	
	testing::InitGoogleTest(&argc, argv);
	setlocale(LC_ALL, "");
	SetConsoleOutputCP(CP_UTF8);  // Установка UTF-8 для вывода в консоль

	g_vk_env = ::testing::AddGlobalTestEnvironment(new WvkBaseTest());

	return RUN_ALL_TESTS();
}