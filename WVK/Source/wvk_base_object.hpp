// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_SOURCE_GPU_PRIVATE___GPU_OBJECT_HPP
#define CGDEV_SOURCE_GPU_PRIVATE___GPU_OBJECT_HPP
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline GpuObject::GpuObject(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline GpuObject::~GpuObject(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline bool GpuObject::create(void) noexcept {

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline void GpuObject::destroy(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// удаляем указатели
					// ~~~~~~~~~~~~~~~~

					for (size_t ct_alloc = 0; ct_alloc < m_alloc.size(); ++ct_alloc) {
						const auto& it_alloc = m_alloc[ct_alloc];

						delete it_alloc;

					}

					// ~~~~~~~~~~~~~~~~
					// очистка данных
					// ~~~~~~~~~~~~~~~~

					m_alloc.clear();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				template<class Object>
				inline WvkStatus GpuObject::allocate(Object** object) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~
					// аллоцируем память через new, 
					// добавляем в std::vector<void*>
					// возвращаем в Object** object
					// ~~~~~~~~~~~~~~~~

					*object = static_cast<Object*>(m_alloc.emplace_back(new Object()));

					return _status;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				template<class Object>
				inline WvkStatus GpuObject::deallocate(Object& object) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~
					// находим,
					// заменяем на последний элемент,
					// последний элемент удаляем
					// ~~~~~~~~~~~~~~~~

					auto _it = m_alloc.begin();

					while (_it != m_alloc.end()) {

						if (*_it == object) {

							delete object;

							object = nullptr;

							auto _last_it = m_alloc.end() - 1;

							if (_last_it != _it) {

								*_it = std::move(*_last_it);
							}

							m_alloc.pop_back();

							return _status;
						}

						++_it;
					};

					// ~~~~~~~~~~~~~~~~
					// находим и удаляем
					// идиома remove/erase
					// ~~~~~~~~~~~~~~~~

					/*m_alloc.erase(
						std::remove_if(
							m_alloc.begin(),
							m_alloc.end(),
							[&object](const auto& element) {

								if (element == object) {

									delete object;

									object = nullptr;

									return true;
								}

								return false;
							}),
						m_alloc.end()
					);*/

					return _status;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE___GPU_OBJECT_HPP
