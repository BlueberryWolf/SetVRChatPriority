// SetPriorityVRChat.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <string>
#include <sstream>
#include <windows.h>
#include "termcolor\termcolor.hpp"

// Global variable declaration: 
// regkey: regkey that is modified
// number: placeholder number, is casted to str (user input)

std::string regkey = "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\VRChat.exe\\PerfOptions";
int number = 0;

// appInfo : a function that displays general app usage info.
void appInfo() {
    std::cout
        << termcolor::bright_blue
        << "EasyAntiCheat prevents you from setting the priority of VRChat while it is running.\n"
        << "This script allows you to set the priority of VRChat "
        << termcolor::underline
        << "on startup.\n"
        << termcolor::reset
        << termcolor::bright_blue
        << "Essentially, bypassing this restriction altogether. \n"
        << termcolor::blue
        << "Note: You have to run this before VRChat. \n\n";

    std::cout
        << termcolor::yellow
        << "Set VRChat Priority:\n"
        << "2: Normal\n"
        << "3: High (recommended)\n"
        << "5: Below Normal\n"
        << "6: Above Normal\n\n"
        << termcolor::red
        << "Realtime priority is not possible due to Windows limitations, and is not recommended for general CPU stability.\n\n"
        << termcolor::reset;
}

// main: main function.

int main() {
    // Local variable declaration:
    // input: user input.
    // strstream: placeholder variable that allows input to be read as a stream, and converted to a number
    // number: input variable as a number

    std::string input;

    SetConsoleTitleA("VRChat Priority");
    system("cls");
    appInfo();

    std::cout << "Input: ";
    std::getline(std::cin, input);

    std::stringstream strstream(input);
    strstream >> number;

    if (number <= 6 && number >= 2) {
        std::string cmd("REG ADD \"" + regkey + "\" /f /v \"CpuPriorityClass\" /t REG_DWORD /d ");
        cmd += input;
        cmd += " >nul";

        system(cmd.c_str());
    }
    else {
        std::cout << "You've entered an invalid number.  Try again.";
        Sleep(1000);
        main();
    }

    std::cout << "Priority of VRChat has been set to: " << input;
    system("pause >nul");

    return 0;
}
