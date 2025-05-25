////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "gpu_object.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "pch.h"

namespace CGDev {

	namespace Tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST(GpuObject, allocate_int) {

			CGDev::GPU::Private::GpuObject _gpu_object;

			int* a = nullptr;

			_gpu_object.allocate(&a);

			*a = 5;

			EXPECT_TRUE(a != nullptr);

		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST(GpuObject, allocate_class_object) {

			CGDev::GPU::Private::GpuObject _gpu_object;

			struct A {

				A() {
					m_b.clear();
				}

				int m_a = 0;
				std::vector<int> m_b;
			};

			A* a = nullptr;

			_gpu_object.allocate(&a);

			a->m_a = 5;

			EXPECT_TRUE(a != nullptr);

		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST(GpuObject, deallocate_0) {

			CGDev::GPU::Private::GpuObject _gpu_object;

			int* a = nullptr;

			_gpu_object.allocate(&a);

			*a = 5;

				int* b = nullptr;

				_gpu_object.allocate(&b);

				*b = 10;

					int* c = nullptr;

					_gpu_object.allocate(&c);

					*c = 15;


			_gpu_object.deallocate(a);

			EXPECT_TRUE(a == nullptr);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST(GpuObject, deallocate_1) {

			CGDev::GPU::Private::GpuObject _gpu_object;

			int* a = nullptr;

			_gpu_object.allocate(&a);

			*a = 5;

				int* b = nullptr;

				_gpu_object.allocate(&b);

				*b = 10;

					int* c = nullptr;

					_gpu_object.allocate(&c);

					*c = 15;

						int* d = new int;

						*d = 20;

			_gpu_object.deallocate(d);

			EXPECT_TRUE(d != nullptr);

		}
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace GPU

} // namespace CGDev
