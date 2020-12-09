local t = 0
local triangle = {}
local transform = {}

function rock.load()
    print("test!")

    --You can use this graphics function anywhere, because
    --it merely modifies the underlying pipeline state's clear colour
    rock.graphics.clear(1, 0, 0, 1)

    local vertices = rock.graphics.test_triangle()
    triangle[1] = rock.graphics.mesh(vertices)
    triangle[2] = rock.graphics.mesh(vertices)

    local pos = rock.math.vec3(0.0,0,0)
    local quat = rock.math.quat_euler(0,0,0)
    local scale = rock.math.vec3(1,1,1)
    transform[1] = rock.math.transform(pos, quat, scale)
    quat = rock.math.quat_euler(0,0,3.14)
    transform[2] = rock.math.transform(pos, quat, scale)
end

function rock.update(dt)
    t = t + dt
    local tmp = rock.math.quat_euler(0,0,t)
    -- transform:setRotation(tmp)
end

function rock.draw()
    rock.graphics.clear(math.sin(t), math.cos(t), 1, 1)
    for i=1,#triangle do
        rock.graphics.draw(triangle[i], transform[i])
    end
end
