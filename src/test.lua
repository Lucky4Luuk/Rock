local t = 0;
local triangle = nil;
local transform = nil;

function rock.load()
    --You can use this graphics function anywhere, because
    --it merely modifies the underlying pipeline state's clear colour
    rock.graphics.clear(1, 0, 0, 1)

    triangle = rock.graphics.mesh()

    pos = rock.math.vec3(30,0,0)
    quat = rock.math.quat_euler(0,0,0)
    scale = rock.math.vec3(1,1,1)
    transform = rock.math.transform(pos, quat, scale)
end

function rock.update(dt)
    t = t + dt
end

function rock.draw()
    rock.graphics.clear(math.sin(t), math.cos(t), 1, 1)
    rock.graphics.draw(triangle, transform)
end
