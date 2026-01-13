// Violettoolssite Worker
// 使用 Cloudflare Workers 全栈部署

export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    
    // API 路由示例（如果需要后端功能）
    if (url.pathname.startsWith('/api/')) {
      return handleAPI(request, url);
    }
    
    // 静态资源由 [assets] 配置自动处理
    // 这里可以添加额外的逻辑，比如访问统计等
    
    return env.ASSETS.fetch(request);
  }
};

// API 处理函数（预留）
async function handleAPI(request, url) {
  const path = url.pathname.replace('/api/', '');
  
  // 示例：获取访问者信息
  if (path === 'visitor') {
    return new Response(JSON.stringify({
      message: 'お可愛いこと',
      timestamp: new Date().toISOString(),
      cf: request.cf
    }), {
      headers: { 
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*'
      }
    });
  }
  
  // 404
  return new Response(JSON.stringify({ error: 'Not Found' }), {
    status: 404,
    headers: { 'Content-Type': 'application/json' }
  });
}

