local function encode_with(whole_bits, decimal_bits)
    return function(values)
        local bits_per_channel = whole_bits // 3;
        local channel_max = 1 << bits_per_channel;
        local pixels = {};

        for _, v in ipairs(values) do
            local clamped_value = math.max(0.0, math.min(v, 1 << decimal_bits));
            local scaled_value = math.floor(v * (1 << (whole_bits - decimal_bits)));

            -- Packs bits like below:
            -- b3 g3 r3 b2 g2 r2 b1 g1 r1 b0 g0 r0
            local int_pixel = {0, 0, 0};
            for b = 0, bits_per_channel - 1 do
                for ch = 1, 3 do
                    local bit_index = b * 3 + (ch - 1);
                    int_pixel[ch] = int_pixel[ch] | (((scaled_value >> bit_index) & 1) << b);
                end
            end

            -- Scale pixel value
            local pixel = {
                int_pixel[1] / channel_max,
                int_pixel[2] / channel_max,
                int_pixel[3] / channel_max,
            };
            table.insert(pixels, pixel);
        end

        return pixels;
    end;
end


local function decode_with(whole_bits, decimal_bits)
    return function(pixels)
        local bits_per_channel = whole_bits // 3;
        local channel_max = 1 << bits_per_channel;
        local values = {};

        for _, pixel in ipairs(pixels) do
            local int_pixel = {
                pixel[1] * channel_max,
                pixel[2] * channel_max,
                pixel[3] * channel_max,
            };

            -- Unpack bits
            local int_value = 0.0;
            for b = 0, bits_per_channel - 1 do
                for ch = 1, 3 do
                    local bit_index = b * 3 + (ch - 1);
                    int_value = int_value | (((int_pixel[ch] >> b) & 1) << bit_index);
                end
            end

            -- Scale value
            local value = int_value / (1 << (whole_bits - decimal_bits));
            table.insert(values, value);
        end

        return values;
    end;
end

return {
    encode_with = encode_with,
    decode_with = decode_with,
};
