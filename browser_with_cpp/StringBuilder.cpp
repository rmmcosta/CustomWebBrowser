#include "StringBuilder.hpp"

StringBuilder::StringBuilder() : m_string("") {}

void StringBuilder::append(const std::string& str) {
    m_string += str;
}

std::string StringBuilder::str() const {
    return m_string;
}