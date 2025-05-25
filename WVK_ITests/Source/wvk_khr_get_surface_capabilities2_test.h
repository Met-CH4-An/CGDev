#pragma once
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "tests.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "Extensions/wvk_khr_get_surface_capabilities2.h"		// что тестируем

#include "wvk_commands_instance_test.h"

namespace CGDev {

	namespace tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkKhrGetSurfaceCapabilities2Test : public ::testing::Test {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void SetUpTestSuite(void) {

				//WvkCommandsInstanceTest::SetUpTestSuite();

				//s_wvk_khr_get_surface_create_info.wvk_commands = WvkCommandsInstanceTest::getTestObject();

				//if (s_wvk_khr_get_surface.getCreateInfo().wvk_commands == nullptr) {
				//	s_wvk_khr_get_surface.create(s_wvk_khr_get_surface_create_info);
				//}
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void TearDownTestSuite(void) {
				//s_wvk_khr_get_surface.destroy();
				//WvkCommandsInstanceTest::TearDownTestSuite();
			}

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//inline static const CGDev::wvk::Extensions::WvkKhrGetSurfaceCapabilities2Ptr getTestObject(void) noexcept {
			//	return &s_wvk_khr_get_surface;
			//}

		protected:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void SetUp() override {
				//m_wvk_khr_get_surface_create_info = s_wvk_khr_get_surface_create_info;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void TearDown() override {
			}

		protected:

			//inline static CGDev::wvk::Extensions::WvkKhrGetSurfaceCapabilities2 s_wvk_khr_get_surface;
			//inline static CGDev::wvk::Extensions::WvkKhrGetSurfaceCapabilities2CreateInfo s_wvk_khr_get_surface_create_info;

			//CGDev::wvk::Extensions::WvkKhrGetSurfaceCapabilities2 m_wvk_khr_get_surface;
			//CGDev::wvk::Extensions::WvkKhrGetSurfaceCapabilities2CreateInfo m_wvk_khr_get_surface_create_info;
		};

	} // namespace tests

} // namespace CGDev
