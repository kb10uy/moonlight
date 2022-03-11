local algebra = require("scripts.algebra");

--- Conversion matrix from RGB to YUV.
local MATRIX_RGB_TO_YUV = {
    {0.299, 0.587, 0.114},
    {-0.168736, -0.331264, 0.5},
    {0.5, -0.418688, -0.081312},
};

--- Conversion matrix from YUV to RGB.
local MATRIX_YUV_TO_RGB = {
    {1.0, 0.0, 1.402},
    {1.0, -0.344136, -0.714136},
    {1.0, 1.772, 0.0},
};

--- Attenuate (RGB444).
---@param pixels table[]
local function rgb444(pixels)
    for _, pixel in ipairs(pixels) do
        pixel[1] = math.floor(pixel[1] * 16.0 + 0.5) / 16.0;
        pixel[2] = math.floor(pixel[2] * 16.0 + 0.5) / 16.0;
        pixel[3] = math.floor(pixel[3] * 16.0 + 0.5) / 16.0;
    end

    return pixels;
end

--- Attenuate (YUV444).
---@param pixels table[]
local function yuv444(pixels)
    local new_pixels = {};
    for _, pixel in ipairs(pixels) do
        local yuv_pixel = algebra.mul_mat3_vec3(MATRIX_RGB_TO_YUV, pixel);
        yuv_pixel[1] = math.floor(yuv_pixel[1] * 16.0 + 0.5) / 16.0;
        yuv_pixel[2] = math.floor((yuv_pixel[2] + 0.5) * 16.0 + 0.5) / 16.0 - 0.5;
        yuv_pixel[3] = math.floor((yuv_pixel[3] + 0.5) * 16.0 + 0.5) / 16.0 - 0.5;
        local rgb_pixel = algebra.mul_mat3_vec3(MATRIX_YUV_TO_RGB, yuv_pixel);
        table.insert(new_pixels, rgb_pixel);
    end

    return new_pixels;
end

--- Attenuate (YUV422 flat).
---@param pixels table[]
local function yuv422_flat(pixels)
    local new_pixels = {};
    for _, pixel in ipairs(pixels) do
        local yuv_pixel = algebra.mul_mat3_vec3(MATRIX_RGB_TO_YUV, pixel);
        yuv_pixel[1] = math.floor(yuv_pixel[1] * 16.0 + 0.5) / 16.0;
        yuv_pixel[2] = math.floor((yuv_pixel[2] + 0.5) * 4.0 + 0.5) / 4.0 - 0.5;
        yuv_pixel[3] = math.floor((yuv_pixel[3] + 0.5) * 4.0 + 0.5) / 4.0 - 0.5;
        local rgb_pixel = algebra.mul_mat3_vec3(MATRIX_YUV_TO_RGB, yuv_pixel);
        table.insert(new_pixels, rgb_pixel);
    end

    return new_pixels;
end

return {
    rgb444 = rgb444,
    yuv444 = yuv444,
    yuv422_flat = yuv422_flat,
};
