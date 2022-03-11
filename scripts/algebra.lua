--- Calculate Mat3 * Vec3.
---@param mat table[]
---@param vec number[]
local function mul_mat3_vec3(mat, vec)
    local result = {0.0, 0.0, 0.0};
    for i = 1, 3 do
        local mat_row = mat[i];
        result[i] = mat_row[1] * vec[1] + mat_row[2] * vec[2] + mat_row[3] * vec[3];
    end
    return result;
end

return {
    mul_mat3_vec3 = mul_mat3_vec3
};
