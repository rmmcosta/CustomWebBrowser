#include "StringBuilder.hpp"

StringBuilder::StringBuilder() : m_string("") {}

void StringBuilder::append(const std::string& str) {
    m_string += str;
}

void StringBuilder::append(char c) {
    m_string += c;
}

std::string StringBuilder::str() const {
    return m_string;
}