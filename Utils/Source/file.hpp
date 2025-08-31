// SPDX-License-Identifier: AGPL-3.0-or-later
#ifndef CGDEV_UTILS_SOURCE__FILE_HPP
#define CGDEV_UTILS_SOURCE__FILE_HPP
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

	namespace utils {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline std::vector<std::byte> loadFileAsChar(const std::string& filename) noexcept {
            std::vector<std::byte> spirv_code;

            std::ifstream file(filename, std::ios::binary | std::ios::ate);
            if (!file) {
				return spirv_code;
            }            

            file.seekg(0, std::ios::end);
            size_t _length = file.tellg();
            file.seekg(0, std::ios::beg);

            spirv_code.resize(_length);
            file.read(reinterpret_cast<char*>(spirv_code.data()), spirv_code.size());
            return spirv_code;
        }

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_UTILS_SOURCE__FILE_HPP
