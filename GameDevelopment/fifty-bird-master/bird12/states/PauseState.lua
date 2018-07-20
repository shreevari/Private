--[[
	PauseState Class
	Author: Shreevari SP
	shreevari.sp@gmail.com

	The PauseState class is the state that the game transitions to when the play is suspended
]]
PauseState = Class{__includes = BaseState}
function PauseState:enter(params)
	self.score = params.score
end

function PauseState:update(dt)
	if love.keyboard.wasPressed('enter') or love.keyboard.wasPressed('return') then
		gStateMachine:change('play')
	end
end
function PauseState:render()
	love.graphics.setFont(mediumFont)
	love.graphics.printf('Score '..tostring(self.score), 0, 100, VIRTUAL_WIDTH, 'center')
	love.graphics.printf('Press enter/return to return to the game again', 0, 160, VIRTUAL_WIDTH, 'center')
end