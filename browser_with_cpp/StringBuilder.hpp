#ifndef STRINGBUILDER_HPP
#define STRINGBUILDER_HPP

#include <string>

class StringBuilder {
public:
    StringBuilder();
    void append(const std::string& str);
    std::string str() const;
private:
    std::string m_string;
};

#endif // STRINGBUILDER_HPP
