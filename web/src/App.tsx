import './App.less';

function App() {

  return (
    <div className="w-full max-w-md px-4">
      <div className="glass rounded-2xl p-5 mb-5 flex flex-col items-center">
        <div className="timer-container flex justify-center items-center">
          <svg className="progress-ring" viewBox="0 0 100 100">
            <circle
              id="work-segment"
              className="segment"
              data-tag="work"
              cx="50"
              cy="50"
              r="45"
              fill="none"
              stroke="#0ea5e9"
              strokeWidth="8"
              strokeDasharray="70.69 141.37"
              strokeDashoffset="0"
            ></circle>
            <circle
              id="study-segment"
              className="segment"
              data-tag="study"
              cx="50"
              cy="50"
              r="45"
              fill="none"
              stroke="#8b5cf6"
              strokeWidth="8"
              strokeDasharray="35.34 141.37"
              strokeDashoffset="-70.69"
            ></circle>
            <circle
              id="reading-segment"
              className="segment"
              data-tag="reading"
              cx="50"
              cy="50"
              r="45"
              fill="none"
              stroke="#10b981"
              strokeWidth="8"
              strokeDasharray="35.34 141.37"
              strokeDashoffset="-106.03"
            ></circle>
            <circle
              id="exercise-segment"
              className="segment"
              data-tag="exercise"
              cx="50"
              cy="50"
              r="45"
              fill="none"
              stroke="#f43f5e"
              strokeWidth="8"
              strokeDasharray="35.34 141.37"
              strokeDashoffset="-141.37"
            ></circle>
            <circle
              cx="50"
              cy="50"
              r="45"
              fill="none"
              stroke="#475569"
              strokeWidth="8"
              strokeDasharray="35.34 141.37"
              strokeDashoffset="-176.71"
            ></circle>
          </svg>
          <div id="tooltip" className="tooltip">
            <div className="tooltip-content glass">
              <div className="font-medium text-white text-sm" id="tooltipTitle">工作</div>
              <div className="text-slate-300 font-mono text-sm" id="tooltipTime">1:25</div>
            </div>
          </div>
          <button
            id="timerBtn"
            className="glass w-32 h-32 rounded-full flex flex-col items-center justify-center shadow-lg hover:bg-slate-700 transition"
          >
            <div id="timer" className="text-3xl font-mono font-bold text-white">25:00</div>
            <div className="text-slate-400 text-xs mt-1" id="timerState">准备开始</div>
          </button>
        </div>
        <div className="timer-info flex flex-row items-center justify-around w-full">
          <span>今日专注：10h</span>
          <span>总专注：3400h</span>
        </div>
      </div>

      <div className="glass rounded-2xl p-4">
        <div className="tag-scroll flex space-x-3 overflow-x-auto p-1">
          <div className="tag-item glass flex-shrink-0 w-16 h-16 rounded-xl flex flex-col items-center justify-center " data-tag="work">
            <i className="fas fa-laptop-code text-primary-400 text-lg mb-1"></i>
            <span className="text-xs text-white">工作</span>
          </div>

          <div className="tag-item glass flex-shrink-0 w-16 h-16 rounded-xl flex flex-col items-center justify-center active" data-tag="study">
            <i className="fas fa-book text-violet-400 text-lg mb-1"></i>
            <span className="text-xs text-white">学习</span>
          </div>

          <div className="tag-item glass flex-shrink-0 w-16 h-16 rounded-xl flex flex-col items-center justify-center" data-tag="reading">
            <i className="fas fa-book-open text-emerald-400 text-lg mb-1"></i>
            <span className="text-xs text-white">阅读</span>
          </div>

          <div className="tag-item glass flex-shrink-0 w-16 h-16 rounded-xl flex flex-col items-center justify-center" data-tag="exercise">
            <i className="fas fa-dumbbell text-rose-400 text-lg mb-1"></i>
            <span className="text-xs text-white">健身</span>
          </div>

          <div className="tag-item glass flex-shrink-0 w-16 h-16 rounded-xl flex flex-col items-center justify-center" data-tag="music">
            <i className="fas fa-music text-amber-400 text-lg mb-1"></i>
            <span className="text-xs text-white">音乐</span>
          </div>
        </div>
      </div>
    </div>
  )
}

export default App;
