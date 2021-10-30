import * as wasm from "crypto_test";
import {provideData} from "./test_data";

export function runBench(benchRuns) {
    if (!Number.isInteger(benchRuns)) {
        return -1;
    }
    let count = 0;
    let seed = [124, 153, 53, 160, 176, 118, 148, 170, 12, 109, 16, 228, 219, 107, 26, 221, 47, 216, 26, 37, 204, 177, 72, 3, 45, 205, 115, 153, 54, 115, 127, 45, 181, 5, 215, 207, 173, 27, 73, 116, 153, 50, 60, 134, 134, 50, 94, 71, 146, 242, 103, 170, 250, 63, 135, 202, 96, 208, 28, 181, 79, 41, 32, 42, 62, 120, 76, 203, 126, 188, 220, 253, 69, 84, 43, 127, 106, 247, 120, 116, 46, 15, 68, 121, 23, 80, 132, 170, 72, 139, 59, 116, 52, 6, 120, 170, 56, 226, 46, 150, 40, 176, 161, 97, 253, 235, 11, 210, 82, 23, 59, 156, 78, 76, 208, 219, 189, 156, 211, 241, 14, 245, 254, 94, 75, 3, 71, 69, 78, 105, 205, 253, 108, 54, 190, 226, 195, 207, 71, 242, 62, 218, 82, 168, 169, 95, 125, 188, 56, 75, 241, 176, 153, 103, 64, 23, 56, 184, 23, 203, 114, 65, 152, 188, 48, 231, 53, 139, 26, 18, 217, 64, 4, 214, 18, 39, 70, 66, 160, 152, 152, 84, 243, 105, 250, 153, 17, 16, 209, 254, 209, 94, 192, 112, 69, 140, 223, 72, 25, 63, 168, 21, 81, 88, 94, 129, 112, 42, 170, 107, 21, 79, 223, 244, 28, 172, 48, 79, 57, 0, 223, 182, 106, 198, 82, 197, 159, 163, 183, 131, 51, 252, 108, 183, 1, 56, 217, 66, 148, 246, 220, 197, 36, 76, 224, 38, 155, 142, 230, 151, 59, 177, 177, 84, 236, 88, 65, 67, 19, 191, 221, 71, 222, 245, 27, 183, 227, 142, 171, 220, 171, 182, 78, 127, 164, 121, 59, 68, 223, 192, 81, 180, 240, 65, 35, 7, 64, 168, 34, 76, 53, 207, 173, 127, 9, 213, 80, 196, 108, 66, 79, 184, 91, 16, 166, 222, 246, 168, 194, 119, 104, 87, 20, 222, 153, 87, 104, 86, 120, 24, 157, 196, 253, 147, 11, 95, 124, 223, 122, 220, 177, 193, 227, 164, 142, 207, 147, 133, 120, 236, 50, 240, 34, 19, 31, 37, 107, 24, 157, 102, 214, 141, 56, 98, 29, 233, 193, 243, 83, 185, 231, 22, 5, 217, 165, 202, 11, 60, 236, 98, 37, 177, 250, 159, 72, 86, 23, 1, 107, 27, 86, 85, 113, 212, 25, 92, 156, 191, 39, 6, 159, 10, 78, 138, 84, 4, 193, 215, 94, 8, 52, 114, 186, 35, 96, 35, 114, 193, 153, 161, 14, 177, 67, 209, 139, 248, 56, 54, 212, 0, 147, 49, 214, 129, 247, 104, 8, 137, 174, 173, 60, 229, 29, 248, 26, 177, 125, 56, 122, 114, 128, 44, 36, 146, 107, 213, 171, 176, 80, 60, 195, 250, 153, 146, 57, 20, 60, 201, 255, 60, 225, 212, 205, 24, 15, 89, 114, 67, 88, 31, 156, 237, 75, 186, 7, 117, 217, 216, 112, 59, 165, 78, 28, 163, 46, 116, 219, 78, 141, 236, 176, 120, 189, 143, 1, 132, 239, 40, 142, 114, 210, 240, 115, 183, 89, 58, 152, 197, 174, 140, 92, 19, 196, 189, 30, 222, 165, 72, 174, 39, 20, 56, 92, 186, 87, 96, 20, 122, 18, 182, 68, 112, 130, 68, 214, 204, 183, 39, 96, 100, 208, 67, 109, 186, 115, 72, 186, 201, 154, 243, 113, 195, 104, 141, 121, 174, 54, 22, 64, 20, 108, 134, 123, 161, 0, 53, 23, 248, 104, 174, 55, 209, 171, 179, 249, 226, 11, 118, 194, 109, 67, 157, 91, 176, 182, 147, 34, 90, 4, 117, 187, 73, 34, 240, 203, 80, 196, 170, 252, 173, 163, 75, 181, 168, 200, 159, 82, 1, 26, 214, 219, 179, 73, 60, 39, 66, 210, 64, 249, 202, 164, 123, 84, 49, 102, 152, 143, 62, 201, 23, 231, 55, 56, 90, 215, 62, 71, 31, 113, 182, 200, 198, 249, 176, 219, 58, 199, 201, 119, 229, 79, 73, 103, 72, 238, 142, 113, 77, 56, 152, 240, 50, 31, 230, 14, 214, 241, 52, 219, 250, 101, 39, 198, 134, 10, 236, 131, 114, 116, 186, 198, 129, 177, 115, 144, 238, 250, 7, 48, 244, 211, 234, 188, 83, 180, 137, 67, 127, 44, 133, 130, 7, 76, 159, 76, 120, 192, 2, 19, 73, 65, 20, 111, 206, 177, 39, 1, 116, 38, 123, 140, 234, 222, 20, 96, 247, 209, 62, 114, 135, 48, 97, 18, 218, 254, 205, 29, 9, 57, 47, 240, 45, 254, 182, 74, 76, 136, 185, 230, 139, 152, 17, 168, 247, 24, 11, 247, 22, 33, 104, 215, 29, 49, 71, 124, 57, 58, 33, 223, 190, 223, 145, 55, 198, 15, 60, 186, 168, 108, 26, 212, 123, 58, 219, 236, 16, 26, 75, 152, 2, 69, 233, 143, 71, 50, 178, 177, 228, 183, 236, 91, 47, 106, 174, 207, 140, 107, 96, 78, 67, 226, 120, 210, 0, 85, 178, 247, 227, 84, 241, 188, 35, 26, 0, 251, 169, 113, 137, 156, 161, 221, 199, 244, 10, 52, 141, 116, 226, 52, 103, 88, 31, 68, 43, 153, 95, 62, 240, 9, 161, 38, 17, 27, 102, 25, 245, 137, 120, 211, 53, 68, 53, 245, 141, 224, 215, 147, 214, 54, 115, 81, 58, 219, 218, 202, 218, 249, 129, 243, 193, 132, 70, 83, 124, 227, 1, 46, 102, 147, 196, 101, 192, 104, 70, 184, 234, 68, 240, 200, 33, 105, 39, 105, 167, 204, 93, 25, 8, 107, 162, 104, 144, 137, 68, 171, 208, 6, 65, 216, 148, 255, 232, 228, 25, 20, 147, 172, 200, 115, 21, 182, 234, 75, 175, 12, 188, 27, 229, 185, 140, 223, 113, 35, 33, 240, 37, 194, 113, 3, 212, 197, 153, 252, 189, 201, 86, 26, 132, 110, 106, 63, 254, 179, 115, 189, 7, 87, 80, 9, 109, 12, 193, 222, 72, 81, 127, 230, 91, 183, 125, 98, 162, 59, 115, 37, 4, 118, 222, 18, 118, 203, 177, 246, 10, 130, 167, 136, 75, 63, 63, 136, 203, 203, 169, 194, 29, 45, 168, 145, 124, 55, 59, 15, 159, 126, 11, 126, 136, 206, 209, 46, 100, 28, 165, 35, 127, 199, 1, 218, 245, 222, 188, 214, 59, 170, 174, 109, 245, 235, 1, 124, 200, 136, 27, 46, 220, 243, 29, 132, 121, 31, 50, 101, 150, 255, 227, 54, 140, 104, 135, 132, 38, 192, 74, 171, 99, 75, 32, 222, 32, 239, 187, 208, 85, 207, 33, 95, 165, 49, 163, 225, 61, 42, 138, 125, 116, 42, 41, 131, 169, 252, 27, 115, 102, 141, 151, 212, 144, 54, 172, 123, 217, 39, 216, 97, 159, 40, 25, 84, 18, 210, 238, 23, 155, 80, 50, 99, 55, 38, 103, 118, 144, 100, 209, 72, 215, 207, 134, 205, 70, 220, 179, 208, 193, 62, 107, 232, 153, 77, 218, 225, 206, 76, 253, 19, 37, 157, 174, 124, 89, 12, 91, 80, 58, 30, 98, 171, 132, 199, 101, 127, 25, 211, 164, 218, 200, 171, 224, 21, 87, 67, 68, 167, 21, 52, 147, 175, 182, 52, 190, 70, 237, 81, 244, 30, 35, 41, 187, 120, 40, 7, 49, 2, 70, 237, 246, 102, 205, 190, 47, 195, 176, 89, 56, 65, 151, 40, 7, 135, 176, 246, 64, 173, 103, 87, 245, 118, 229, 136, 212, 44, 38, 32, 194, 226, 111, 95, 114, 33, 220, 79, 176, 9, 202, 9, 130, 244, 206, 157, 155, 150, 107, 209, 152, 231, 61, 182, 230, 45, 166, 194, 1, 140, 69, 200, 9, 10, 27, 184, 104, 32, 32, 47, 178, 8, 235, 51, 48, 111, 220, 236, 131, 26, 17, 1, 137, 213, 45, 88, 181, 73, 234, 199, 44, 170, 168, 235, 241, 221, 63, 72, 84, 82, 21, 69, 95, 34, 32, 176, 200, 117, 4, 21, 222, 184, 219, 194, 179, 22, 68, 253, 141, 20, 39, 72, 128, 40, 16, 53, 187, 68, 124, 43, 151, 231, 95, 28, 178, 81, 88, 243, 31, 236, 41, 196, 14, 43, 42, 173, 82, 107, 59, 246, 181, 200, 71, 183, 255, 182, 132, 212, 245, 112, 75, 242, 235, 92, 14, 30, 198, 37, 59, 1, 196, 177, 126, 171, 158, 15, 99, 132, 2, 127, 186, 182, 220, 97, 127, 227, 159, 212, 123, 231, 56, 232, 169, 67, 21, 21, 201, 95, 97, 59, 57, 131, 29, 31, 191, 41, 123, 180, 67, 130, 159, 70, 253, 130, 34, 30, 236, 46, 208, 42, 76, 103, 245, 108, 149, 209, 192, 94, 112, 10, 117, 243, 87, 155, 111, 165, 26, 90, 185, 120, 149, 89, 150, 16, 228, 193, 102, 105, 146, 181, 168, 15, 188, 39, 191, 194, 254, 65, 245, 122, 156, 66, 158, 123, 92, 148, 107, 45, 124, 114, 123, 235, 22, 121, 220, 8, 239, 155, 233, 177, 149, 197, 17, 122, 227, 111, 193, 231, 23, 97, 90, 245, 253, 70, 76, 196, 139, 22, 48, 91, 23, 224, 113, 136, 24, 239, 241, 129, 223, 235, 144, 101, 4, 172, 127, 50, 97, 0, 61, 179, 177, 181, 242, 64, 38, 167, 216, 175, 139, 212, 44, 112, 62, 121, 254, 213, 177, 3, 12, 160, 81, 7, 94, 0, 109, 100, 226, 240, 40, 4, 25, 153, 123, 169, 54, 121, 167, 34, 74, 144, 156, 201, 44, 130, 21, 94, 130, 173, 4, 220, 46, 202, 72, 80, 148, 188, 4, 240, 210, 68, 103, 30, 105, 148, 182, 102, 36, 15, 152, 128, 126, 107, 61, 240, 174, 38, 255, 24, 28, 111, 149, 163, 42, 2, 141, 167, 166, 76, 82, 112, 4, 5, 134, 78, 106, 23, 120, 51, 124, 18, 50, 116, 196, 73, 204, 44, 60, 226, 245, 26, 69, 42, 34, 218, 39, 70, 201, 118, 88, 134, 103, 97, 189, 93, 202, 237, 87, 164, 67, 205, 151, 82, 53, 35, 169, 165, 65, 43, 225, 175, 255, 192, 168, 180, 26, 125, 238, 127, 106, 139, 77, 122, 19, 247, 219, 182, 233, 163, 49, 180, 120, 166, 53, 185, 156, 209, 7, 164, 99, 45, 109, 249, 82, 242, 242, 83, 8, 9, 94, 178, 199, 12, 186, 87, 118, 121, 181, 155, 126, 101, 12, 78, 252, 148, 98, 176, 138, 145, 219, 75, 171, 84, 153, 199, 6, 106, 211, 173, 28, 206, 158, 77, 49, 191, 158, 154, 129, 160, 254, 32, 224, 182, 244, 71, 38, 223, 55, 19, 250, 88, 20, 226, 214, 236, 235, 77, 238, 210, 108, 26, 119, 67, 60, 77, 149, 122, 6, 111, 45, 157, 182, 178, 127, 193, 40, 29, 166, 100, 56, 206, 123, 201, 81, 12, 154, 121, 214, 248, 156, 89, 101, 96, 27, 37, 103, 124, 124, 242, 114, 8, 43, 167, 246, 242, 15, 32, 12, 213, 12, 1, 109, 224, 187, 193, 232, 129, 163, 26, 250, 49, 139, 204, 131, 214, 163, 81, 156, 165, 160, 49, 204, 174, 48, 29, 180, 160, 197, 72, 94, 233, 86, 173, 189, 142, 140, 5, 26, 150, 248, 106, 12, 230, 198, 137, 189, 139, 91, 194, 44, 238, 107, 220, 247, 109, 207, 125, 83, 251, 230, 51, 103, 250, 5, 80, 235, 33, 247, 105, 128, 144, 10, 115, 102, 30, 174, 13, 153, 232, 194, 77, 99, 158, 250, 105, 34, 252, 182, 196, 120, 163, 177, 119, 16, 35, 247, 0, 250, 250, 72, 19, 254, 255, 190, 241, 182, 176, 70, 212, 166, 237, 70, 194, 117, 187, 13, 218, 200, 62, 79, 222, 254, 81, 24, 104, 246, 106, 234, 43, 236, 51, 218, 153, 204, 3, 234, 24, 72, 227, 149, 161, 91, 173, 200, 34, 241, 40, 144, 144, 13, 28, 58, 136, 122, 208, 161, 178, 77, 160, 107, 28, 234, 21, 71, 112, 151, 15, 121, 43, 104, 95, 230, 142, 56, 238, 188, 14, 168, 50, 50, 134, 214, 165, 105, 111, 6, 10, 198, 197, 114, 140, 151, 60, 123, 93, 84, 93, 66, 83, 164, 91, 75, 159, 60, 27, 40, 121, 236, 183, 97, 21, 44, 171, 47, 72, 157, 142, 177, 96, 164, 238, 174, 240, 85, 198, 23, 80, 133, 44, 108, 164, 196, 22, 248, 27, 215, 92, 154, 234, 177, 189, 130, 26, 42, 23, 226, 40, 59, 248, 39, 53, 207, 35, 161, 248, 222, 93, 38, 31, 216, 111, 202, 227, 127, 127, 26, 117, 180, 13, 251, 250, 153, 87, 170, 153, 138, 173, 29, 167, 71, 20, 196, 62, 36, 226, 207, 92, 187, 154, 195, 1, 145, 21, 13, 169, 174, 28, 33, 192, 118, 95, 255, 162, 53, 123, 153, 131, 171, 246, 44, 197, 181, 95, 251, 242, 128, 73, 254, 42, 106, 150, 1, 139, 224, 216, 127, 35, 58, 48, 248, 80, 34, 212, 150, 31, 187, 130, 122, 110, 48, 223, 47, 139, 149, 231, 142, 85, 186, 73, 229, 46, 234, 52, 173, 52, 225, 239, 188, 106, 29, 103, 64, 222, 209, 24, 113, 29, 27, 73, 221, 102, 37, 71, 96, 65, 78, 136, 236, 112, 174, 242, 226, 165, 116, 117, 165, 94, 60, 74, 253, 190, 246, 22, 6, 92, 86, 173, 119, 155, 30, 212, 55, 172, 31, 226, 77, 222, 207, 78, 49, 93, 207, 208, 5, 50, 91, 24, 68, 38, 203, 141, 118, 138, 217, 248, 1, 113, 244, 225, 194, 221, 215, 226, 97, 42, 240, 144, 145, 219, 160, 91, 21, 4, 47, 13, 59, 249, 59, 139, 16, 199, 238, 214, 231, 44, 54, 219, 145, 207, 124, 226, 168, 188, 41, 149, 148, 229, 198, 166, 98, 99, 24, 125, 246, 218, 171, 223, 152, 128, 155, 41, 39, 109, 88, 56, 86, 58, 134, 224, 189, 39, 225, 94, 29, 8, 52, 129, 227, 4, 37, 243, 120, 10, 238, 205, 175, 251, 125, 169, 121, 57, 27, 105, 23, 200, 229, 35, 110, 26, 19, 39, 224, 24, 57, 127, 235, 248, 57, 128, 134, 154, 224, 99, 221, 40, 240, 226, 223, 206, 162, 130, 181, 90, 56, 171, 39, 63, 44, 29, 247, 250, 20, 99, 231, 90, 242, 213, 240, 202, 212, 12, 113, 201, 10, 35, 32, 88, 174, 255, 143, 230, 151, 45, 115, 70, 5, 163, 94, 241, 244, 0, 98, 69, 213, 188, 86, 148, 187, 51, 213, 193, 189, 12, 76, 30, 119, 153, 207, 2, 40, 27, 227, 40, 168, 180, 112, 170, 33, 197, 114, 203];
    let pk   = [234, 212, 209, 175, 247, 128, 213, 174, 175, 89, 13, 115, 180, 75, 1, 200, 228, 91, 195, 185, 238, 192, 185, 2, 144, 199, 188, 238, 211, 56, 73, 243, 253, 214, 186, 61, 44, 52, 239, 86, 82, 252, 171, 159, 118, 147, 14, 228, 155, 68, 138, 196, 148, 223, 188, 24, 42, 120, 241, 13, 215, 0, 20, 239, 154, 97, 130, 45, 2, 73, 122, 24, 116, 69, 46, 243, 45, 243, 215, 173, 86, 189, 82, 24, 143, 80, 252, 33, 211, 183, 130, 169, 11, 243, 146, 118, 223, 53, 154, 56, 11, 208, 217, 42, 255, 157, 146, 108, 68, 195, 66, 119, 49, 207, 101, 240, 149, 86, 54, 192, 232, 67, 163, 171, 4, 189, 144, 67, 208, 49, 157, 139, 68, 129, 125, 217, 116, 81, 78, 115, 240, 153, 60, 130, 191, 1, 202, 54, 29, 183, 162, 1, 249, 64, 51, 145, 162, 237, 249, 89, 222, 94, 72, 152, 9, 54, 200, 125, 136, 1, 247, 68, 143, 68, 83, 192, 125, 200, 91, 254, 146, 221, 109, 188, 148, 192, 164, 92, 105, 232, 23, 81, 23, 83, 107, 190, 112, 210, 200, 225, 248, 96, 178, 90, 175, 96, 148, 2, 170, 128, 219, 237, 83, 97, 10, 117, 194, 56, 171, 129, 32, 130, 169, 55, 164, 108, 75, 240, 66, 161, 128, 154, 248, 162, 180, 5, 219, 192, 121, 171, 127, 115, 24, 149, 166, 60, 69, 50, 141, 249, 43, 214, 242, 98, 209, 123, 187, 165, 133, 120, 130, 118, 42, 37, 181, 141, 128, 111, 239, 180, 52, 85, 131, 64, 40, 219, 222, 127, 44, 120, 179, 42, 37, 185, 238, 75, 20, 76, 169, 246, 50, 244, 142, 77, 246, 232, 208, 130, 215, 121, 93, 97, 117, 204, 74, 87, 30, 38, 136, 78, 99, 10, 126, 115, 77, 221, 82, 45, 147, 89, 43, 249, 226, 165, 84, 112, 225, 34, 38, 229, 228, 176, 61, 203, 7, 35, 201, 145, 32, 39, 229, 34, 152, 179, 151, 122, 136, 182, 52, 37, 131, 187, 253, 10, 200, 198, 80, 48, 53, 175, 15, 194, 158, 37, 234, 163, 197, 68, 243, 176, 242, 193, 201, 140, 6, 242, 3, 114, 197, 149, 25, 206, 48, 117, 13, 213, 85, 253, 17, 42, 62, 64, 125, 145, 196, 19, 26, 65, 28, 198, 147, 41, 222, 34, 151, 31, 28, 116, 98, 80, 220, 91, 215, 95, 71, 138, 225, 139, 196, 230, 15, 82, 2, 150, 217, 87, 129, 135, 136, 109, 230, 78, 224, 188, 45, 80, 104, 136, 175, 178, 60, 94, 54, 56, 198, 216, 124, 169, 62, 158, 217, 53, 77, 21, 21, 180, 28, 162, 205, 159, 161, 224, 48, 89, 245, 211, 105, 239, 207, 190, 31, 119, 168, 118, 7, 172, 16, 45, 216, 151, 30, 203, 147, 61, 124, 26, 254, 85, 112, 128, 94, 57, 230, 77, 160, 162, 115, 86, 25, 20, 210, 112, 100, 33, 18, 229, 179, 175, 111, 107, 48, 111, 199, 159, 117, 254, 68, 96, 40, 66, 18, 228, 134, 146, 164, 15, 36, 98, 160, 105, 213, 39, 210, 226, 131, 151, 38, 134, 49, 137, 52, 176, 144, 167, 210, 87, 217, 139, 19, 155, 253, 164, 3, 16, 103, 241, 17, 6, 90, 32, 43, 52, 149, 85, 78, 188, 135, 104, 24, 2, 168, 241, 128, 18, 174, 36, 77, 248, 174, 78, 89, 217, 122, 233, 17, 211, 32, 24, 221, 253, 69, 27, 214, 125, 138, 51, 28, 161, 36, 13, 235, 47, 109, 63, 111, 93, 114, 111, 121, 148, 106, 213, 36, 143, 54, 34, 102, 190, 163, 51, 221, 31, 202, 27, 144, 103, 24, 93, 199, 137, 0, 134, 4, 20, 80, 90, 50, 67, 20, 52, 230, 247, 204, 134, 192, 104, 248, 114, 222, 172, 201, 210, 57, 221, 193, 201, 134, 49, 147, 201, 161, 59, 145, 73, 43, 162, 59, 19, 228, 198, 211, 59, 186, 67, 224, 80, 145, 10, 103, 150, 16, 155, 77, 196, 25, 135, 40, 117, 9, 67, 165, 202, 106, 107, 118, 19, 172, 64, 14];
    let sk   = [208, 103, 217, 143, 0, 85, 226, 195, 222, 239, 16, 118, 187, 183, 85, 175, 208, 101, 17, 44, 133, 196, 110, 108, 53, 6, 85, 217, 98, 80, 115, 233, 78, 69, 40, 192, 83, 231, 32, 32, 108, 219, 120, 10, 97, 173, 81, 32, 196, 152, 164, 205, 62, 96, 195, 52, 216, 112, 47, 15, 121, 216, 20, 24, 175, 149, 156, 171, 119, 34, 235, 221, 48, 223, 10, 71, 158, 20, 3, 211, 55, 173, 146, 62, 93, 188, 237, 78, 224, 72, 57, 2, 15, 119, 81, 192, 117, 164, 137, 42, 171, 25, 220, 123, 210, 174, 145, 39, 178, 65, 131, 29, 24, 55, 185, 226, 36, 48, 206, 25, 37, 159, 120, 165, 197, 191, 208, 229, 54, 94, 91, 174, 229, 41, 24, 204, 171, 229, 131, 181, 15, 0, 18, 109, 4, 44, 188, 220, 56, 98, 153, 238, 104, 238, 78, 236, 216, 96, 230, 19, 20, 34, 174, 230, 178, 0, 115, 55, 88, 200, 45, 196, 190, 35, 79, 108, 70, 53, 111, 1, 126, 125, 7, 228, 91, 220, 201, 172, 209, 175, 221, 221, 92, 145, 56, 128, 82, 206, 10, 83, 107, 146, 22, 6, 181, 234, 166, 144, 44, 164, 239, 57, 20, 38, 17, 249, 187, 86, 109, 79, 215, 84, 20, 67, 85, 13, 140, 96, 178, 246, 153, 16, 115, 201, 161, 248, 4, 58, 164, 105, 233, 18, 123, 106, 47, 94, 189, 150, 128, 13, 36, 184, 185, 213, 134, 31, 17, 150, 177, 200, 5, 103, 110, 164, 103, 139, 18, 67, 115, 143, 154, 233, 122, 197, 104, 188, 91, 106, 224, 120, 199, 189, 79, 97, 86, 129, 114, 8, 97, 60, 139, 89, 207, 123, 122, 242, 168, 251, 233, 99, 124, 91, 118, 67, 78, 101, 241, 179, 97, 164, 246, 187, 247, 225, 253, 228, 125, 214, 221, 42, 83, 32, 29, 145, 14, 98, 195, 225, 26, 176, 194, 61, 136, 119, 222, 199, 29, 39, 247, 129, 70, 238, 211, 153, 164, 224, 99, 190, 192, 193, 117, 227, 171, 65, 142, 8, 237, 33, 135, 93, 162, 95, 166, 123, 192, 54, 67, 211, 183, 11, 124, 26, 17, 140, 21, 59, 58, 245, 142, 213, 196, 205, 26, 127, 50, 54, 77, 19, 52, 164, 77, 52, 153, 16, 79, 170, 218, 85, 85, 184, 102, 148, 119, 46, 240, 6, 95, 198, 115, 176, 117, 118, 181, 48, 133, 194, 113, 173, 179, 6, 90, 108, 42, 93, 158, 119, 74, 190, 148, 175, 190, 184, 182, 1, 171, 111, 11, 97, 154, 127, 133, 88, 209, 168, 36, 255, 79, 185, 70, 232, 92, 109, 27, 156, 143, 85, 100, 79, 209, 179, 47, 142, 140, 143, 156, 229, 71, 93, 144, 79, 27, 30, 1, 118, 49, 17, 244, 210, 13, 34, 146, 121, 85, 206, 108, 242, 173, 194, 147, 202, 75, 134, 159, 149, 16, 126, 149, 14, 237, 149, 8, 138, 248, 98, 166, 30, 201, 205, 210, 250, 51, 226, 195, 39, 12, 184, 200, 156, 58, 66, 215, 251, 45, 156, 227, 218, 253, 123, 107, 233, 172, 22, 203, 253, 218, 5, 11, 37, 91, 197, 6, 77, 79, 234, 142, 128, 34, 138, 83, 60, 229, 61, 108, 73, 255, 143, 218, 203, 225, 29, 184, 225, 98, 192, 161, 250, 185, 36, 251, 186, 65, 186, 196, 248, 126, 89, 203, 249, 236, 40, 178, 207, 227, 200, 94, 225, 76, 30, 237, 21, 174, 8, 36, 253, 107, 195, 159, 180, 88, 243, 154, 139, 208, 107, 176, 82, 247, 160, 227, 122, 167, 58, 80, 241, 99, 15, 234, 65, 138, 157, 112, 145, 108, 243, 104, 165, 81, 194, 251, 71, 65, 80, 85, 133, 168, 108, 49, 42, 231, 137, 199, 221, 144, 35, 18, 72, 22, 249, 32, 147, 23, 243, 188, 222, 0, 138, 159, 48, 199, 220, 155, 189, 4, 126, 83, 88, 247, 187, 182, 135, 214, 214, 28, 73, 233, 86, 214, 247, 22, 219, 184, 220, 248, 68, 89, 118, 86, 111, 58, 48, 128, 143, 186, 95, 88, 64, 100, 25, 138, 157, 138, 204, 176, 66, 13, 91, 14, 81, 162, 26, 26, 237, 100, 223, 179, 118, 98, 198, 55, 248, 62, 252, 84, 117, 41, 88, 143, 93, 52, 107, 165, 152, 253, 243, 112, 164, 213, 255, 140, 169, 188, 42, 158, 52, 5, 239, 90, 220, 225, 78, 220, 42, 154, 91, 159, 80, 33, 107, 55, 112, 109, 86, 29, 126, 76, 246, 41, 235, 173, 113, 43, 226, 202, 87, 95, 12, 21, 182, 91, 70, 167, 67, 214, 219, 197, 86, 127, 84, 91, 72, 94, 47, 9, 6, 162, 208, 90, 162, 234, 86, 253, 61, 151, 101, 150, 11, 218, 13, 58, 91, 26, 5, 56, 38, 112, 217, 1, 181, 59, 111, 68, 171, 146, 64, 218, 127, 27, 234, 232, 28, 168, 194, 121, 13, 29, 242, 168, 177, 136, 18, 241, 178, 247, 124, 131, 142, 193, 120, 151, 88, 165, 182, 255, 104, 22, 33, 251, 115, 88, 154, 237, 239, 67, 6, 44, 15, 153, 193, 95, 183, 121, 246, 158, 188, 163, 148, 68, 235, 151, 188, 136, 2, 246, 38, 200, 177, 93, 230, 102, 204, 181, 202, 82, 97, 46, 161, 213, 148, 232, 174, 214, 44, 4, 0, 192, 169, 67, 120, 90, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 67, 67, 120, 90, 85, 0, 0, 176, 110, 108, 180, 255, 127, 0];
    let ct   = [185, 52, 135, 43, 4, 105, 52, 156, 195, 200, 138, 229, 122, 249, 141, 110, 35, 48, 230, 102, 206, 136, 158, 55, 7, 82, 50, 36, 137, 28, 6, 133, 147, 63, 3, 20, 178, 142, 29, 110, 28, 53, 89, 200, 105, 94, 242, 106, 182, 186, 58, 247, 1, 246, 63, 226, 176, 100, 46, 137, 109, 23, 247, 30, 234, 134, 248, 193, 173, 24, 194, 180, 181, 247, 244, 210, 244, 236, 164, 145, 85, 13, 9, 151, 23, 120, 53, 96, 223, 196, 202, 233, 183, 67, 81, 231, 26, 75, 26, 43, 224, 1, 184, 197, 255, 105, 172, 242, 94, 138, 235, 1, 3, 156, 21, 199, 162, 155, 174, 99, 186, 103, 73, 221, 108, 184, 59, 60, 38, 37, 143, 85, 241, 20, 41, 217, 31, 233, 12, 67, 136, 181, 35, 123, 10, 46, 183, 20, 58, 49, 169, 229, 12, 161, 96, 130, 198, 29, 27, 83, 26, 234, 201, 6, 13, 93, 125, 183, 91, 13, 251, 122, 88, 105, 103, 176, 71, 79, 158, 84, 214, 145, 103, 249, 241, 193, 70, 17, 137, 148, 28, 111, 188, 176, 251, 234, 15, 201, 117, 94, 61, 230, 9, 215, 133, 201, 184, 208, 113, 253, 106, 144, 200, 39, 237, 13, 131, 80, 181, 7, 5, 44, 140, 44, 67, 102, 63, 129, 207, 54, 132, 212, 40, 65, 181, 29, 199, 7, 133, 152, 24, 188, 187, 83, 70, 45, 31, 19, 236, 165, 65, 209, 6, 202, 91, 117, 106, 111, 244, 130, 230, 224, 246, 85, 160, 223, 127, 187, 85, 132, 48, 233, 95, 187, 182, 213, 142, 240, 122, 230, 190, 42, 36, 176, 119, 25, 67, 185, 230, 129, 57, 148, 160, 160, 61, 76, 247, 96, 41, 171, 161, 203, 30, 163, 144, 70, 159, 148, 77, 122, 35, 181, 11, 129, 99, 224, 180, 40, 30, 43, 118, 158, 142, 54, 163, 193, 99, 133, 206, 175, 148, 129, 182, 193, 92, 246, 2, 199, 154, 237, 160, 203, 194, 98, 120, 204, 144, 68, 72, 179, 96, 177, 179, 44, 89, 94, 167, 225, 62, 167, 45, 93, 238, 58, 11, 191, 232, 1, 72, 30, 39, 97, 57, 177, 102, 58, 193, 247, 231, 162, 173, 135, 246, 23, 249, 3, 221, 76, 163, 210, 77, 136, 119, 2, 67, 95, 189, 235, 20, 159, 85, 149, 242, 118, 165, 241, 108, 110, 59, 91, 253, 99, 241, 252, 229, 248, 50, 25, 119, 156, 203, 20, 179, 163, 76, 238, 24, 197, 218, 24, 99, 89, 107, 64, 84, 250, 38, 58, 108, 148, 100, 236, 210, 70, 232, 236, 238, 138, 157, 33, 66, 0, 66, 85, 186, 251, 5, 157, 104, 126, 130, 199, 245, 178, 80, 224, 175, 56, 140, 246, 95, 53, 142, 197, 56, 51, 184, 223, 246, 160, 149, 45, 68, 60, 24, 146, 154, 88, 137, 244, 160, 53, 47, 63, 1, 191, 81, 47, 227, 115, 96, 181, 236, 215, 202, 244, 5, 83, 1, 182, 107, 53, 42, 88, 161, 153, 231, 147, 100, 182, 149, 113, 172, 214, 68, 2, 220, 130, 240, 219, 0, 43, 197, 103, 89, 87, 135, 233, 102, 172, 189, 142, 176, 215, 58, 73, 74, 102, 33, 121, 103, 236, 167, 193, 31, 228, 128, 251, 179, 66, 210, 253, 167, 60, 204, 44, 189, 185, 21, 129, 90, 79, 129, 95, 97, 116, 157, 79, 120, 18, 232, 146, 72, 170, 132, 233, 252, 35, 48, 17, 51, 129, 192, 93, 5, 215, 129, 142, 184, 136, 205, 96, 49, 21, 191, 96, 154, 160, 223, 71, 189, 19, 98, 69, 100, 147, 39, 47, 175, 178, 223, 240, 95, 59, 250, 155, 204, 89, 29, 42, 152, 147, 84, 228, 2, 47, 97, 29, 3, 251, 0, 31, 162, 56, 120, 196, 35, 226, 121, 74, 151, 77, 220, 52, 136, 205, 151, 147, 128, 82, 254, 113, 207, 197, 249, 62, 175, 150, 246, 182, 112, 9, 76, 13, 59, 41, 67, 64, 229, 199, 58, 60, 186, 184, 6, 169, 128, 58, 138, 64, 218, 120, 140, 126, 153, 123, 210, 27, 2];
    let ss   = [23, 111, 219, 176, 9, 221, 63, 132, 139, 54, 90, 183, 241, 141, 156, 12, 145, 114, 25, 49, 200, 89, 76, 44, 111, 4, 60, 134, 0, 121, 26, 108];

    let runTimes = [];
    let chunk = provideData(1);
    for (let i = 0; i < 1; i++) {
        console.log("Benchmark run " + i);
        let startTime = performance.now();
        // wasm.ntru_bench_with_parameters(chunk[0][2], chunk[0][3]);
        wasm.ntru_bench();
        let endTime = performance.now();
        runTimes.push(endTime - startTime);
        console.log("Benchmark run " + i + " finished");
    }
    return runTimes;
}