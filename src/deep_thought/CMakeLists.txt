add_library(deep_thought)

target_sources(deep_thought
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/include/deep_thought/answer.hpp
        ${CMAKE_CURRENT_SOURCE_DIR}/src/answer.cpp
)

target_include_directories(deep_thought
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/include
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/src
)
