/**
 * cnmsb AI 补全 - Cloudflare Workers 代理
 * 
 * 部署说明：
 * 1. 在 Cloudflare Workers 中创建新的 Worker
 * 2. 复制此代码到 Worker 中
 * 3. 在 Worker 设置中绑定 AI（Workers AI）
 *    - 变量名称：cnmsb
 *    - 类型：Workers AI
 * 4. 部署 Worker 并获取 URL
 * 5. 配置 cnmsb 使用该 URL：
 *    cnmsb ai-config set base_url "https://your-worker.your-subdomain.workers.dev/"
 *    cnmsb ai-config set api_key "any-value"  # Cloudflare Workers AI 不需要 API Key，但 cnmsb 要求设置
 *    cnmsb ai-config set model "@cf/qwen/qwen1.5-14b-chat-awq"
 */

export default {
  async fetch(request, env) {
    // 处理 CORS 预检请求
    if (request.method === "OPTIONS") {
      return new Response(null, {
        headers: {
          "Access-Control-Allow-Origin": "*",
          "Access-Control-Allow-Methods": "POST, OPTIONS",
          "Access-Control-Allow-Headers": "Content-Type, Authorization",
        },
      });
    }

    // 只处理 POST 请求
    if (request.method !== "POST") {
      return new Response(JSON.stringify({ error: "Method not allowed" }), {
        status: 405,
        headers: { "Content-Type": "application/json" },
      });
    }

    try {
      const body = await request.json();
      const { model, messages } = body;

      // 使用绑定的 AI (变量名: cnmsb)
      const response = await env.cnmsb.run(model || "@cf/qwen/qwen1.5-14b-chat-awq", {
        messages: messages,
      });

      // 转换为 OpenAI 兼容格式
      const openaiResponse = {
        id: "chatcmpl-" + Date.now(),
        object: "chat.completion",
        created: Math.floor(Date.now() / 1000),
        model: model || "@cf/qwen/qwen1.5-14b-chat-awq",
        choices: [
          {
            index: 0,
            message: {
              role: "assistant",
              content: response.response,
            },
            finish_reason: "stop",
          },
        ],
        usage: {
          prompt_tokens: 0,
          completion_tokens: 0,
          total_tokens: 0,
        },
      };

      return new Response(JSON.stringify(openaiResponse), {
        headers: {
          "Content-Type": "application/json",
          "Access-Control-Allow-Origin": "*",
        },
      });
    } catch (error) {
      return new Response(
        JSON.stringify({
          error: {
            message: error.message || "Internal server error",
            type: "server_error",
          },
        }),
        {
          status: 500,
          headers: {
            "Content-Type": "application/json",
            "Access-Control-Allow-Origin": "*",
          },
        }
      );
    }
  },
};

