local function encode(values)
    local pixels = {};

    local pixel = {0.0, 0.0, 0.0};
    for i, v in ipairs(values) do
        local index = (i - 1) % 3 + 1;
        pixel[index] = math.min(math.max(0.0, v), 1.0);
        if index == 3 then
            table.insert(pixels, pixel);
            pixel = {0.0, 0.0, 0.0};
        end
    end

    if #values % 3 ~= 0 then
        table.insert(pixels, pixel);
    end

    return pixels;
end

local function decode(pixels)
    local values = {};

    for _, v in ipairs(pixels) do
        table.insert(values, v[1]);
        table.insert(values, v[2]);
        table.insert(values, v[3]);
    end

    return values;
end

return {
    encode = encode,
    decode = decode,
};
